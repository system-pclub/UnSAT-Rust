from typing import Any
from verirule.errors import EquivalenceError, RuleError
from verirule.schema import validate_rule
import z3




def make_z3_vars(variables: list[dict[str, Any]]) -> dict[str, z3.ExprRef]:
    env: dict[str, z3.ExprRef] = {}

    for var in variables:
        name = var["name"]
        sort = var["sort"]

        if sort == "Int":
            env[name] = z3.Int(name)
        elif sort == "Bool":
            env[name] = z3.Bool(name)
        elif sort == "Real":
            env[name] = z3.Real(name)
        else:
            # For equivalence over buggy_violation, most selector variables
            # should have scalar sorts. If you need Pointer/Usize, model them
            # as uninterpreted sorts separately.
            raise EquivalenceError(f"Unsupported variable sort for Z3 variable: {sort}")

    return env


def expr_to_z3(expr: dict[str, Any], env: dict[str, z3.ExprRef]) -> z3.ExprRef:
    if "var" in expr:
        name = expr["var"]
        if name not in env:
            raise EquivalenceError(f"Unknown variable: {name}")
        return env[name]

    if "int" in expr:
        return z3.IntVal(expr["int"])

    if "bool" in expr:
        return z3.BoolVal(expr["bool"])

    if "string" in expr:
        # Usually not useful for arithmetic constraints.
        return z3.StringVal(expr["string"])

    if "op" not in expr:
        raise EquivalenceError(f"Invalid expression: {expr}")

    op = expr["op"]
    args = [expr_to_z3(arg, env) for arg in expr["args"]]

    if op == "=":
        return args[0] == args[1]
    if op == "!=":
        return args[0] != args[1]
    if op == ">":
        return args[0] > args[1]
    if op == ">=":
        return args[0] >= args[1]
    if op == "<":
        return args[0] < args[1]
    if op == "<=":
        return args[0] <= args[1]

    if op == "+":
        return args[0] + args[1]
    if op == "-":
        return args[0] - args[1]
    if op == "*":
        return args[0] * args[1]
    if op == "/":
        return args[0] / args[1]

    if op == "and":
        return z3.And(*args)
    if op == "or":
        return z3.Or(*args)
    if op == "not":
        return z3.Not(args[0])
    if op == "=>":
        return z3.Implies(args[0], args[1])

    raise EquivalenceError(f"Unsupported op: {op}")


def are_equivalent(
    variables: list[dict[str, Any]],
    expr_a: dict[str, Any],
    expr_b: dict[str, Any],
    context: list[dict[str, Any]] | None = None,
) -> tuple[bool, z3.ModelRef | None]:
    """
    Return:
      (True, None) if expr_a and expr_b are equivalent under context.
      (False, model) if Z3 finds a counterexample.
    """

    env = make_z3_vars(variables)

    z3_a = expr_to_z3(expr_a, env)
    z3_b = expr_to_z3(expr_b, env)

    solver = z3.Solver()

    if context:
        for c in context:
            solver.add(expr_to_z3(c, env))

    # Find a case where A and B differ.
    solver.add(z3.Xor(z3_a, z3_b))

    result = solver.check()

    if result == z3.unsat:
        return True, None

    if result == z3.sat:
        return False, solver.model()

    raise EquivalenceError("Z3 returned unknown")

def normalize_variables(variables: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {v["name"]: v for v in variables}

def compare_variable_sets(
    rule_a: dict[str, Any],
    rule_b: dict[str, Any],
) -> list[str]:
    errors: list[str] = []

    vars_a = normalize_variables(rule_a["variables"])
    vars_b = normalize_variables(rule_b["variables"])

    names_a = set(vars_a)
    names_b = set(vars_b)

    for name in sorted(names_a - names_b):
        errors.append(f"Variable missing in B: {name}")

    for name in sorted(names_b - names_a):
        errors.append(f"Variable missing in A: {name}")

    for name in sorted(names_a & names_b):
        if vars_a[name]["sort"] != vars_b[name]["sort"]:
            errors.append(
                f"Variable {name} sort differs: "
                f"A={vars_a[name]['sort']}, B={vars_b[name]['sort']}"
            )

    return errors

def model_to_dict(
    model: z3.ModelRef,
    env: dict[str, z3.ExprRef],
) -> dict[str, str]:
    result: dict[str, str] = {}

    for name, var in env.items():
        result[name] = str(model.eval(var, model_completion=True))

    return result

def check_context_sat(
    *,
    variables: list[dict[str, Any]],
    context: list[dict[str, Any]],
) -> tuple[bool, dict[str, str] | None]:
    env = make_z3_vars(variables)
    solver = z3.Solver()

    for expr in context:
        solver.add(expr_to_z3(expr, env))

    result = solver.check()

    if result == z3.sat:
        return True, model_to_dict(solver.model(), env)

    if result == z3.unsat:
        return False, None

    raise RuleError("Z3 returned unknown while checking context satisfiability.")


def compare_buggy_violation_equivalence(
    rule_a: dict[str, Any],
    rule_b: dict[str, Any],
    *,
    use_context: bool = True,
) -> dict[str, Any]:
    validate_rule(rule_a)
    validate_rule(rule_b)

    variable_errors = compare_variable_sets(rule_a, rule_b)
    if variable_errors:
        return {
            "equivalent": None,
            "reason": "Variable sets are not compatible.",
            "errors": variable_errors,
        }

    variables_by_name = normalize_variables(rule_a["variables"])
    variables = list(variables_by_name.values())

    context: list[dict[str, Any]] = []
    if use_context:
        context.extend(rule_a.get("preconditions", []))
        context.extend(rule_a.get("operation_semantics", []))

    if use_context:
        context_sat, _ = check_context_sat(variables=variables, context=context)
        if not context_sat:
            return {
                "equivalent": None,
                "reason": "Context is unsatisfiable, so equivalence would be vacuous.",
            }

    env = make_z3_vars(variables)

    expr_a = expr_to_z3(rule_a["buggy_violation"], env)
    expr_b = expr_to_z3(rule_b["buggy_violation"], env)

    solver = z3.Solver()

    for expr in context:
        solver.add(expr_to_z3(expr, env))

    solver.add(z3.Xor(expr_a, expr_b))

    result = solver.check()

    if result == z3.unsat:
        return {
            "equivalent": True,
            "reason": "No assignment satisfies A XOR B under the given context.",
        }

    if result == z3.sat:
        model = solver.model()
        return {
            "equivalent": False,
            "reason": "Z3 found an assignment where A and B evaluate differently.",
            "counterexample": model_to_dict(model, env),
            "a_value": str(model.eval(expr_a, model_completion=True)),
            "b_value": str(model.eval(expr_b, model_completion=True)),
        }

    return {
        "equivalent": None,
        "reason": "Z3 returned unknown.",
    }