import os
from utils import parse_file, process_project_folder


def traverse(node, callback, caller=None, line_offset=0):
    if node.type == 'function_item':
        caller = node.children[1].text
        line_offset = node.start_point[0]
    callback(node, caller, line_offset)
    for child in node.children:
        traverse(child, callback, caller, line_offset)


def find_calls(node, caller, line_offset):
    if node.type == 'call_expression':
        callee = node.children[0].text
        callee_line = node.start_point[0] + line_offset
        print(f"Function '{caller}' at line {callee_line} calls function '{callee}'.")


def get_all_type_tree(root_folder, output_folder):
    # Parse all source code files according to the project structure to obtain the .tree files.
    process_project_folder(root_folder, output_folder)


if __name__ == '__main__':
    file_path = "zkevm-circuits/zkevm-circuits/src/evm_circuit/util/math_gadget/constant_division.rs"
    tree = parse_file(file_path)
    root_node = tree.root_node
    traverse(root_node, find_calls)
