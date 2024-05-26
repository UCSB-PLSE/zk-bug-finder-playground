import os
from tree_sitter import Language, Parser

# build the parser for different languages
# Language.build_library("build/rust.so", ['tree-sitter-rust'])

RUST_LANGUAGE = Language("build/rust.so", "rust")


def parse_file(file_path):
    lang = None
    lang = RUST_LANGUAGE

    if lang:
        # Initialize the parser
        parser = Parser()
        parser.set_language(lang)

        with open(file_path, 'r') as file:
            source_code = file.read()

        # Parse the source code
        tree = parser.parse(bytes(source_code, 'utf-8'))
        return tree


def read_callable_byte_offset(byte_offset, src):
    return src[byte_offset: byte_offset + 1]


def print_tree(node, depth=0):
    if node.child_count == 0:
        return

    start_row, start_col = node.start_point
    end_row, end_col = node.end_point

    print('  ' * depth + f'{node.type} ({start_row}, {start_col}) - ({end_row}, {end_col}):')
    for i in range(node.child_count):
        print_tree(node.child(i), depth + 1)


def write_tree(output_file, node, depth=0):
    if node.child_count == 0:
        return

    start_row, start_col = node.start_point
    end_row, end_col = node.end_point
    with open(output_file, 'a') as file:
        file.write('  ' * depth + f'{node.type}: {node.text} ({start_row}, {start_col}) - ({end_row}, {end_col}):\n')
    for i in range(node.child_count):
        write_tree(output_file, node.child(i), depth + 1)


def parse_source_code_file(file_path, root_folder, output_folder):
    tree = parse_file(file_path)

    relative_path = os.path.relpath(file_path, start=root_folder)

    # Create the output folder if it doesn't exist
    os.makedirs(os.path.join(output_folder, os.path.dirname(relative_path)), exist_ok=True)

    # Store the tree structure in a new file in the output folder
    output_file = os.path.join(output_folder, f'{relative_path}.tree')
    print("output_file:" + output_file)
    with open(output_file, 'w') as out:
        pass
    write_tree(output_file, tree.root_node)


def process_project_folder(root_folder, output_folder):
    # Parse all source code files according to the project structure to obtain the .tree files.
    for root, dirs, files in os.walk(root_folder):
        for file_name in files:
            if file_name.endswith(('.rs')):
                file_path = os.path.join(root, file_name)
                print(file_path)
                parse_source_code_file(file_path, root_folder, output_folder)