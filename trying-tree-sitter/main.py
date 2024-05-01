from utils import process_project_folder

if __name__ == '__main__':
    root_folder = '/home/zhongyouwei/ucsb/zkevm-circuits/zkevm-circuits'
    output_folder = '/home/zhongyouwei/ucsb/zk-bug-finder-playground/trying-tree-sitter/tree-sitter-result/zkevm-circuits'
    process_project_folder(root_folder, output_folder)
