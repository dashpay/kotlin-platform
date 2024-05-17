# ignore _ctor and _destroy functions
import re

header_file_path = '../dash-sdk-bindings/target/dash_sdk_bindings.h'
output_file_path = 'src/main/swig/ignore.i'

def is_complete_declaration(line):
    """Check if the line ends with a semicolon, indicating a complete declaration."""
    return line.strip().endswith(';')

def format_arguments(args_str):
    """Simple formatting to ensure consistency in argument lists."""
    args = args_str.split(',')
    formatted_args = [ ' '.join(arg.strip().split()) for arg in args ]
    return ', '.join(formatted_args)

def should_ignore(declaration):
    return '_ctor' in declaration or '_destroy' in declaration #or '_set_0' in declaration or '_get_0' in declaration

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

def extract_functions(header_file, output_file):
    with open(header_file, 'r') as file:
        lines = file.readlines()

    declaration = ""
    with open(output_file, 'w') as outfile:
        for line in lines:
            declaration += " " + line.strip()
            if is_complete_declaration(line):
                ignore_directive = process_declaration(declaration)
                if ignore_directive:
                    outfile.write(ignore_directive)
                declaration = ""  # Reset for the next declaration


extract_functions(header_file_path, output_file_path)