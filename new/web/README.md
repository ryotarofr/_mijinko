## ブロック AST/LSP

ライブラリ側の実装
ユーティリティ
- `split_lines`: \n で区切る
- ローカル変更でのバージョン管理

アプリケーション側の実装
- 変更保存時にバージョン管理


## TODO
コードブロック内は `biome` の ESLint が作動するようにできないかな。
イメージとしては、トップレベルでは指定した言語でエディタを記述する。
マークダウン(そのほかの言語ではコメントアウト)の時は、コードブロックで、別の ESlint を実行できるのが理想。


plane なマークダウン olly のモード
と
直感的なプレビューをリアルタイムで表示するモード(一旦やらない)
→ こっちのほうは @lexical/React では root ref にcreateHtmlElement してコンテンツ変更で useCallback, useLayoutEffect, useMemo が発火する仕組みにしていた。つまり、文字入力のたびにレンダリングさせないと無理ってこと。


すべての入力に対してLocalHistryを作る必要があるのか？
→ Lexicalではやってないのでいらないはず
ctrl + z, ctrl + shift + z と混同してはいけない
→ この機能は別で作る必要がある。
