use core::time::Duration;
use jni::errors::Result;
use jni::objects::{JObject, JString};
use jni::strings::JavaStr;
use jni::sys::{jbooleanArray, jdouble, jdoubleArray, jint, jlong};
use jni::JNIEnv;
use ordered_float::NotNan;
use rtlola_frontend::ParserConfig;
use rtlola_interpreter::{EvalConfig, Incremental, Monitor, TimeFormat, TimeRepresentation, Value};
use std::ffi::CStr;

/// Represents the monitor, should only be an opaque pointer in Kotlin.
pub struct KotlinMonitor {
    monitor: Monitor<Incremental>,
    relevant_ixs: Vec<usize>,
    num_inputs: usize,
}

/// Initializes a monitor for a given spec.
///
/// The `spec` is a string representation of the specification. The `relevant_output` argument is a string containing
/// the names of all relevant output streams, separated by commas.  Only the outputs of these streams will be reported by the monitor.
#[no_mangle]
pub extern "C" fn init(
    env: JNIEnv,
    _: JObject,
    spec: JString,
    relevant_outputs: JString,
) -> *const KotlinMonitor {
    let spec = unsafe { convert_java_str(env.get_string(spec).unwrap()) };
    let relevant_outputs = unsafe { convert_java_str(env.get_string(relevant_outputs).unwrap()) };

    let ir = rtlola_frontend::parse(ParserConfig::for_string(spec)).unwrap();
    let ec = EvalConfig::api(TimeRepresentation::Relative(TimeFormat::HumanTime));

    let relevant_ixs = relevant_outputs
        .split(',')
        .map(|name| {
            ir.outputs
                .iter()
                .find(|o| o.name == name)
                .expect("ir does not contain required output stream")
                .reference
                .out_ix()
        })
        .collect();

    let num_inputs = ir.inputs.len();
    let m: Monitor<Incremental> = rtlola_interpreter::Config::new_api(ec, ir).as_api();
    let monitor = KotlinMonitor {
        monitor: m,
        relevant_ixs,
        num_inputs,
    };

    Box::into_raw(Box::new(monitor))
}

/// Receives a single event and returns an array of verdicts.
///
/// Interprets the `monitor` input as pointer to a `KotlinMonitor` received via the `init` function.
/// The `input` argument contains a long value for each input of the specification plus the current timestamp at the end.
#[no_mangle]
pub extern "C" fn receive_single_value(
    env: JNIEnv,
    _: JObject,
    monitor: jlong,
    input_ix: jint,
    value: jdouble,
    timestamp: jdouble,
) -> jdoubleArray {
    let mut mon = unsafe { Box::from_raw(monitor as *mut KotlinMonitor) };
    let mut event = vec![Value::None; mon.num_inputs];
    event[input_ix as usize] = value_from_f64(value);
    process_event(env, &mut mon, &event, timestamp)
}

/// Receives a single event and returns an array of verdicts.
///
/// Interprets the `monitor` input as pointer to a `KotlinMonitor` received via the `init` function.
/// The `input` argument contains a long value for each input of the specification plus the current timestamp at the end.
#[no_mangle]
pub extern "C" fn receive_total_event(
    env: JNIEnv,
    _: JObject,
    monitor: jlong,
    inputs: jdoubleArray,
) -> jdoubleArray {
    let mut mon = unsafe { Box::from_raw(monitor as *mut KotlinMonitor) };
    let num_values = mon.num_inputs + 1;
    let inputs = get_floats(env, inputs, num_values);

    debug_assert!(inputs.is_ok());
    if inputs.is_err() {
        // In release config, ignore invalid inputs.
        return env.new_double_array(0).unwrap();
    }
    let inputs = inputs.unwrap();
    let (time, inputs) = inputs.split_last().unwrap();
    let inputs = inputs
        .iter()
        .copied()
        .map(value_from_f64)
        .collect::<Vec<_>>();
    process_event(env, &mut mon, &inputs, *time)
}

/// Receives a single event and returns an array of verdicts.
///
/// Interprets the `monitor` input as pointer to a `KotlinMonitor` received via the `init` function.
/// The `input` argument contains a long value for each input of the specification plus the current timestamp at the end.
/// The `active` argument is a bool array where a `true` value at position `ix` indicates that the `ix`th value of
/// `input` contains a meaningful new value.  All other values will be ignored.
/// The timestamp must always be active, so the following invariant must hold:
/// `len(inputs) == len(active) && last(active) || len(inputs) == len(active) + 1
#[no_mangle]
pub extern "C" fn receive_partial_event(
    env: JNIEnv,
    _: JObject,
    monitor: jlong,
    inputs: jdoubleArray,
    active: jbooleanArray,
) -> jdoubleArray {
    let mut mon = unsafe { Box::from_raw(monitor as *mut KotlinMonitor) };
    let num_values = mon.num_inputs + 1;

    let inputs = get_floats(env, inputs, num_values);
    let active = get_bools(env, active, num_values);
    // crash in debug
    debug_assert!(inputs.is_ok());
    debug_assert!(active.is_ok());
    if active.is_err() || inputs.is_err() {
        // In release config, ignore invalid inputs.
        return env.new_double_array(0).unwrap();
    }
    let inputs = inputs.unwrap();
    let (time, input) = inputs.split_last().unwrap();
    let active = active.unwrap();

    let event: Vec<Value> = input
        .iter()
        .zip(active)
        .map(|(f, a)| if a { value_from_f64(*f) } else { Value::None })
        .collect();
    process_event(env, &mut mon, &event, *time)
}

fn get_floats(env: JNIEnv, inputs: jdoubleArray, num_values: usize) -> Result<Vec<f64>> {
    let mut event = vec![0.0; num_values];
    env.get_double_array_region(inputs, 0, &mut event)?;
    Ok(event)
}

fn get_bools(
    env: JNIEnv,
    inputs: jdoubleArray,
    num_values: usize,
) -> Result<impl Iterator<Item = bool>> {
    let mut values = vec![0u8; num_values];
    env.get_boolean_array_region(inputs, 0, &mut values)?;
    Ok(values.into_iter().map(|v| v != 0))
}

fn process_event(env: JNIEnv, mon: &mut KotlinMonitor, event: &[Value], time: f64) -> jdoubleArray {
    let updates = mon
        .monitor
        .accept_event(event, Duration::new(time.floor() as u64, 0));

    let num_updates = updates.timed.len();
    let res = env
        .new_double_array((num_updates * mon.relevant_ixs.len()) as i32)
        .unwrap();
    let output_copy_res: jni::errors::Result<()> =
        updates
            .timed
            .iter()
            .enumerate()
            .try_for_each(|(ix, update)| {
                let (_, values) = update;
                let output: Vec<jdouble> = values
                    .iter()
                    .filter(|(sr, _v)| mon.relevant_ixs.contains(sr))
                    .map(|(_sr, v)| {
                        if let Value::Float(f) = v {
                            f.into_inner()
                        } else {
                            0f64
                        }
                    })
                    .collect();
                env.set_double_array_region(res, (mon.relevant_ixs.len() * ix) as i32, &output)
            });
    debug_assert!(output_copy_res.is_ok());
    res
}

fn value_from_f64(v: f64) -> Value {
    Value::Float(NotNan::new(v).unwrap())
}

unsafe fn convert_java_str(js: JavaStr) -> String {
    let raw = CStr::from_ptr(js.as_ptr());
    String::from(raw.to_str().unwrap())
}
