#[macro_export]
macro_rules! async_call {

    // for enum(struct)
    ($tx:expr, $($enum_name:ident)::+ {
        $( $field:ident $(: $value:expr)? $(,)?)*
     }) => {{
         use tokio::sync::oneshot;
         let (ret, get) = oneshot::channel();
         let _ = $tx.send(  $($enum_name)::+ {
             $(
                 $field $(:$value)?,
             )*
             ret,
         });
         get
     }};


    // for enum(tuple)
    ($tx:expr,  $($enum_name:ident)::+ (
        $( $field:expr  $(,)? )*
    )) => {{
         use tokio::sync::oneshot;
         let (ret, get) = oneshot::channel();
         let _ = $tx.send($($enum_name)::+ (
             $(
                $field,
            )*
             ret,
         ));
         get
     }};

    // for struct
     ($tx:expr, $struct_name:ident {
        $( $field:ident $(:$value:expr)? $(,)?)*
     }) => {{
         use tokio::sync::oneshot;
         let (ret, get) = oneshot::channel();
         let _ = $tx.send( $struct_name {
            $(
                $field $(:$value)?,
            )*
             ret,
         });
         get
     }};
}
