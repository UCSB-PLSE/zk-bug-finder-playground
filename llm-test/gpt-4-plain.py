import json

# 加载你的 JSON 数据
your_data = json.load(open('gpt-4-plain-urls.json', 'r'))

# 解析出 URL ID
url_ids = {data['query_url'].split('/')[-1]: data['id'] for data in your_data}

# 加载 shared_conversations.json 数据
with open('shared_conversations.json', 'r') as file:
    shared_data = json.load(file)

# 查找对应的 conversation_id
conversation_ids = {url_ids[item['id']]: item['conversation_id'] for item in shared_data if item['id'] in url_ids}

file.close()

with open('conversations.json', 'r') as file:
   conversations = json.load(file)

file.close()

# 从 conversations.json 中提取对应的 conversation，并存入以相应 id 命名的 json 文件中
for conversation_id in conversation_ids.items():
    for conversation in conversations:
        if conversation['id'] == conversation_id[1]:
            # 以 conversation_id 对应的 key 而非 value 命名
            with open(f'{conversation_id[0]}.json', 'w') as file:
                json.dump(conversation, file)