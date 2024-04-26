from utils import parse_file


def find_call_expressions(node, result, func_node):
    if node.type == 'call_expression':
        if node.children[0].text.decode('utf-8').__contains__("cb.account_access_list_write"):
            result.append(node)

    for child in node.children:
        find_call_expressions(child, result, func_node)


def traverse_function(node, result):
    if node.type == 'function_item':
        call_expressions = []
        find_call_expressions(node, call_expressions, node)
        if call_expressions:
            result.append(node)
    for child in node.children:
        traverse_function(child, result)






if __name__ == '__main__':
    file_path = "/Users/tangken/Desktop/ucsb/web3/zkevm-circuits/zkevm-circuits/src/evm_circuit/execution/callop.rs"
    tree = parse_file(file_path)
    root_node = tree.root_node
    fun_list = []
    traverse_function(root_node, fun_list)
    print(len(fun_list))
    for fun in fun_list:

        pass
