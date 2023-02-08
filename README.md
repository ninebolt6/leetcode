## 環境構築

1. [LeetCode Extension](https://marketplace.visualstudio.com/items?itemName=LeetCode.vscode-leetcode) を VSCode にインストール
2. settings.json に以下を設定

```json
{
  "leetcode.defaultLanguage": "rust",
  "leetcode.workspaceFolder": "PATH_TO_WORKSPACE_FOLDER",
  "leetcode.filePath": {
    "default": {
      "folder": "src/problems",
      "filename": "${snake_case_name}_${id}.${ext}"
    }
  }
}
```

## 問題を解く

1. `Show Problem`を押してファイルを生成する
2. 生成されたファイルの行頭に`use crate::Solution`を追加
3. 実装する
4. 提出する
