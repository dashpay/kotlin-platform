# ignore _ctor and _destroy functions
import re

header_file_path = '../dash-sdk-bindings/target/include/dash_sdk_bindings.h'
output_file_path = 'src/main/swig/ignore.i'
clone_file_path = 'src/main/cpp/clone.h'

def is_complete_declaration(line):
    """Check if the line ends with a semicolon, indicating a complete declaration."""
    return line.strip().endswith(';')

def format_arguments(args_str):
    """Simple formatting to ensure consistency in argument lists."""
    args = args_str.split(',')
    formatted_args = [ ' '.join(arg.strip().split()) for arg in args ]
    return ', '.join(formatted_args)

def should_ignore(declaration):
    if 'platform_mobile_sdk_destroy_dash_sdk' in declaration:
        return False
    return '_ctor' in declaration or '_destroy' in declaration or '_clone' in declaration

def should_do_clone(declaration):
    return '_clone' in declaration


def process_declaration(declaration):
    """Process a complete function declaration to extract relevant ignore directives."""
    if should_ignore(declaration):
        # Simplified extraction process
        parts = declaration.split('(')
        pre_args = parts[0].split()
        func_name = pre_args[-1]
        if func_name.startswith('*'):
            func_name = func_name[1:]
        args_str = parts[1].split(')')[0]
        formatted_args = format_arguments(args_str)
        return f'%ignore {func_name}({formatted_args});\n'
    return ""

def process_clone(declaration):
    if should_do_clone(declaration):
        parts = declaration.split('(')
        pre_args = parts[0].split()
        func_name = pre_args[-1]
        if func_name.startswith('*'):
            func_name = func_name[1:]
        args_str = parts[1].split('*')[0]
        formatted_args = format_arguments(args_str)
        return f'inline {formatted_args} * clone({formatted_args} * object) {{\n    return (object != nullptr) ? {func_name}(object) : nullptr;\n}}\n'

    return ""

def extract_functions(header_file, output_file, clone_file):
    with open(header_file, 'r') as file:
        lines = file.readlines()

    declaration = ""
    with open(output_file, 'w') as outfile:
        with open(clone_file, 'w') as clonefile:
            for line in lines:
                declaration += " " + line.strip()
                if is_complete_declaration(line):
                    ignore_directive = process_declaration(declaration)
                    if ignore_directive:
                        outfile.write(ignore_directive)
                    clone_directive = process_clone(declaration)
                    if clone_directive:
                        clonefile.write(clone_directive)

                    declaration = ""  # Reset for the next declaration


extract_functions(header_file_path, output_file_path, clone_file_path)