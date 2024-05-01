
from utils import parse_file
import os

inputs = []
outputs = []
unconstrained = []
gadget = b''

def traverse(node, callback, line_offset=0):
    for child in node.children:
        callback(child, line_offset)

def find_calls(node, line_offset):
    global gadget
    global inputs
    global outputs
    global unconstrained
    if node.type == 'struct_item':
        struct_name = node.children[2].text
        gadget = struct_name
        decoded_gadget = gadget.decode()
        if 'Gadget' in decoded_gadget:
            inputs.clear()
            outputs.clear()
            unconstrained.clear()
            # print(f"pub struct '{struct_name}' defined at line {node.start_point[0] + line_offset}.")
            for child in node.children:
                if child.type == 'field_declaration_list':
                    for field in child.children:
                        if field.type == 'field_declaration':
                            if field.children[0].text == b'pub':
                                outputs.append(field.children[1].text)
                                # print(f"Input '{field.children[1].text}' defined at line {field.start_point[0] + line_offset}.")
                            else:
                                outputs.append(field.children[0].text)
                            # print(f"Output '{field.children[0].text}' defined at line {field.start_point[0] + line_offset}.")
        else:
            gadget = b''
    if gadget and node.type == 'impl_item':
        impl_name = node.children[2].text
        # print(f"impl '{impl_name}' defined at line {node.start_point[0] + line_offset}.")
        decoded_impl_name = impl_name.decode()
        decoded_gadget = gadget.decode()
        if decoded_gadget and (decoded_impl_name.startswith(decoded_gadget)):
            # print(f"decoded_gadget: {decoded_gadget}, decoded_impl_name: {decoded_impl_name}")
            for child in node.children:
                if child.type == 'declaration_list':
                    for declaration in child.children:
                        if declaration.type == 'function_item':
                            function_name = declaration.children[2].text
                            decoded_function_name = function_name.decode()
                            if 'construct' in decoded_function_name:
                                for func_child in declaration.children:
                                    if func_child.type == 'parameters':
                                        for param in func_child.children:
                                            if param.type == 'parameter':
                                                inputs.append(param.children[0].text)
                                                # print(f"Input '{param.children[0].text}' defined at line {param.start_point[0] + line_offset}.")
                                    if func_child.type == 'block':
                                        for block_child in func_child.children:
                                            if block_child.type == 'let_declaration':
                                                right_value = block_child.children[3].text
                                                decoded_right_value = right_value.decode()
                                                new_constrained = unconstrained.copy()
                                                for unconstrained_var in unconstrained:
                                                    decoded_unconstrained_var = unconstrained_var.decode()
                                                    if decoded_unconstrained_var in decoded_right_value:
                                                        # print(f"Removed unconstrained variable '{unconstrained_var}'")
                                                        new_constrained.remove(unconstrained_var)
                                                unconstrained = new_constrained.copy()
                                                variable_name = block_child.children[1].text
                                                decoded_variable_name = variable_name.decode()
                                                if decoded_variable_name == 'mut':
                                                    variable_name = block_child.children[2].text
                                                    decoded_variable_name = variable_name.decode()
                                                if variable_name not in inputs and variable_name not in outputs:
                                                    # print(variable_name)
                                                    unconstrained.append(variable_name)
                                                    # print(f"Intermediate variable '{variable_name}' defined at line {block_child.start_point[0] + line_offset} is temporarily added.")
                                            if block_child.type == 'expression_statement':
                                                new_constrained = unconstrained.copy()
                                                for unconstrained_var in unconstrained:
                                                    right_value = block_child.children[0].text
                                                    decoded_right_value = right_value.decode()
                                                    decoded_unconstrained_var = unconstrained_var.decode()
                                                    # print(decoded_right_value)
                                                    if decoded_unconstrained_var in decoded_right_value:
                                                        new_constrained.remove(unconstrained_var)
                                                unconstrained = new_constrained.copy()
        
            if gadget != '' and unconstrained:
                print(f"unconstraint variables in '{gadget}': {unconstrained}")
                unconstrained.clear()


def parse_source_code_file(file_path):
    tree = parse_file(file_path)
    root_node = tree.root_node
    traverse(root_node, find_calls)

def process_project_folder(root_folder):
    # Parse all source code files according to the project structure to obtain the .tree files.
    for root, dirs, files in os.walk(root_folder):
        for file_name in files:
            if file_name.endswith(('.rs')):
                file_path = os.path.join(root, file_name)
                print(file_path)
                parse_source_code_file(file_path)

root_folder = '/home/zhongyouwei/ucsb/zkevm-circuits/zkevm-circuits'
process_project_folder(root_folder)

# file_path = '/home/zhongyouwei/ucsb/zkevm-circuits/zkevm-circuits/src/evm_circuit/util/memory_gadget.rs'
# parse_source_code_file(file_path)