# FE Router(随時更新)

## '/'

home_editor.rsを表示

## '/memo/:id'

`vim FILENAME` で一致するものがある場合に侵入

`FILENAME`に紐づく`memo_id`がURIの`:id`の部分で使う

↑
cd 時にファイルが存在するかをチェックすることで一意性を担保できる
