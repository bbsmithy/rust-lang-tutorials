mod variables;
mod data_types;
mod functions;
mod control_flow;
mod fibonacci_example;

fn variables_example(example: variables::Examples){
    match example {
        variables::Examples::ImmutableSuccess => variables::immutable_success(),
        variables::Examples::Shadowing => variables::shadowing(),
    }
}

fn data_types_example(example: data_types::Examples){
    match example {
        data_types::Examples::Integers => data_types::integers(),
        data_types::Examples::FloatingPoint => data_types::floating_point(),
        data_types::Examples::NumericOperations => data_types::numeric_operations(),
        data_types::Examples::BoolType => data_types::bool_type(),
        data_types::Examples::CharType => data_types::char_type(),
        data_types::Examples::TupleType => data_types::tuple_type(),
        data_types::Examples::ArrayType => data_types::array_type(),
    }
}

fn functions_example(example: functions::Examples) -> i32 {
    match example {
        functions::Examples::ImplicitReturn => functions::implicit_return(6),
        functions::Examples::TypedParamaters => functions::typed_paramaters(8)
    }
}

fn control_flow_example(example: control_flow::Examples){
    match example {
        control_flow::Examples::IfLet => control_flow::if_let(false),
        control_flow::Examples::Loop => control_flow::loopy_job(),
        control_flow::Examples::While => control_flow::while_job(),
        control_flow::Examples::For => control_flow::for_job(),
        control_flow::Examples::For_Range => control_flow::for_range_job(),
    }
}


fn main() {
    variables_example(variables::Examples::Shadowing);
    data_types_example(data_types::Examples::TupleType);
    functions_example(functions::Examples::TypedParamaters);
    control_flow_example(control_flow::Examples::For_Range);
    //Fibonacci sequence to 13th item
    for num in (0..13){
        println!("{}", fibonacci_example::fib_job(num));
    }

}
