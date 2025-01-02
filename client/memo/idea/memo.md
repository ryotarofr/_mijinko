## 記憶

- 字句解析器に関しての実装まとめる
  - ML言語の特徴に関連させる

## オートコンプリートについて(理想論)

### IME保管機能はIMEのみで良いか？

とりあえすIMEのみで考える

### 保存方法

Jsonで保存

画面遷移時に非同期でロードする

### 保管候補の動的生成について

補完候補を特定のJsonから取得

入力条件によりmatchしたやつのみを絞り込む形にする

例. 「だいにっぽんていこく」と入力後スペースキー押下 「大日本帝国」

```json
// ユーザ単位で持っているJson
{
  "imeInputs": [
    {
      "original": "こんにちは",
      "converted": "コンニチハ",
      "timestamp": "2024-12-15T12:00:00Z",
      "frequency": 1
    },
    {
      "original": "ありがとう",
      "converted": "アリガトウ",
      "timestamp": "2024-12-15T12:05:00Z",
      "frequency": 2
    }
  ]
}

// 共通のJson
{
  "imeInputs": [
    {
      "original": "だいにっぽんていこく",
      "converted": ["大日本帝国", "ダイニッポンテイコク", "ﾀﾞｲﾆｯﾎﾟﾝﾃｲｺｸ"],
      "timestamp": "2024-12-15T12:00:00Z",
      "frequency": 1
    },
    {
      "original": "ありがとう",
      "converted": "アリガトウ",
      "timestamp": "2024-12-15T12:05:00Z",
      "frequency": 2
    }
  ]
}
```

```rs
use diamond_types::list::{ListBranch, ListOpLog};
use dioxus::prelude::*;

let mut oplog = use_signal(ListOpLog::new);
let agent = oplog.write().get_or_create_agent_id("ryotarofr");

oplog.write().add_insert(agent, 0, "Hello World!!!");

info!("{:?}", oplog);
```

oplogの中身

```json
 OpLog { doc_id: None, client_with_localtime: RleVec([KVPair(0, CRDTSpan { agent: 0, seq_range: T 0..14 })]), client_data: [ClientData { name: "ryotarofr", item_times: RleVec([KVPair(0, T 0..14)]) }], operation_ctx: OperationCtx { ins_content: [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 33, 33], del_content: [] }, operations: RleVec([KVPair(0, OperationInternal { loc: RangeRev { span: T 0..14, fwd: true }, kind: Ins, content_pos: Some(T 0..14) })]), history: History { entries: RleVec([HistoryEntry { span: T 0..14, shadow: 4294967295, parents: [], child_indexes: [] }]), root_child_indexes: [0] }, version: [13] }
 ```

### 最適化について

(一旦は考えない)

変換後、確定した文字を保存しておく

### 使い道

応用: データの統計分析や再利用

- オートコンプリート候補の学習データとして利用。
- ユーザーごとの入力傾向を分析。
- 他のデバイス間で同期。

### 機械学習を活用

より高度なオートコンプリートを実現するには、保存したデータを機械学習モデルに組み込むことを検討します。

シンプルな学習手法: Nグラムモデル
Nグラムモデルは、入力履歴から次に来る単語やフレーズを予測するのに使われます。
例:
「ありがとう」を入力した後に「ございます」を提案する。
「おはよう」と入力した後に「ございます」が候補に出る。

### ユーザー体験の最適化

- 動的更新:
  - ユーザーが入力するたびに候補をリアルタイムで更新。
- キャッシュの活用:
  - 頻繁に使用される候補をキャッシュして、高速表示を実現。
- カスタマイズ:
  - ユーザーごとに個別の履歴や傾向に基づいた候補を提案。

## マークダウン

例えばコードブロック内は独自のフォーマッターを適応させるとか

## 誤字検知機能

誤字の警告出してくれとか

## データ

 editor_state :EditorState { contents: EditorData { lines: [LINE<Text("a") line_type: Normal>, LINE<Text("a")Text("a")Text("a")Text("a")Cursor line_type: Normal>, LINE< line_type: Normal>] }, current_line: 2, cursor_position: 5, selection_start: 0, selection_end: 0 }

{ contents: EditorData { lines: [LINE<Text("s")Text("a")Text("s")Text("a")>, LINE<>, LINE<Text("-")Char('\u{a0}')>, LINE<Text("-\u{a0}")>, LINE<Text("-\u{a0}")Text("#")Text("#")Char('\u{a0}')Text("s")Text("a")Text("s")Text("a")Text("s")>, LINE<Text("#")Char('\u{a0}')Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Text("s")Cursor>, LINE<>] }
}

このデータでlinesにはテキストデータが入っています。行どとにカンマ区切りで挿入されています。仮にデータないの検索性を高めるために、カンマごとに行数のidを割り振った方が効率は上がりますか？

#### chatGPT

カンマごとに行IDを割り振る（つまり各行に一意の識別子を付与する）ことで検索効率が向上するかどうかは、主に下記のような条件や利用手法によります。

1. **検索手法の違い**:  
   - **全文検索（キーワード検索）**の場合、行IDを振っただけでは、直接的な検索速度向上は限定的です。結局は全文を走査したり、インデックス（例えばインバーテッドインデックス）を作成する必要があるためです。  
   - **ランダムアクセスが必要な検索**、つまり「特定の行番号やIDで指定された行を瞬時に参照したい」場合には、あらかじめ行IDを付与したデータ構造を用意し、行IDから該当行へ素早くアクセスできるようにしておくことで、確実に検索（アクセス）効率が上がります。

2. **データの格納形式や利用環境**:
   - テキストファイルをそのまま扱うような場合、カンマで分離された文字列を逐次走査するより、あらかじめ「行ID: テキスト」というマッピング（ハッシュマップやインデックスファイル）を作成しておけば、特定IDで引けるためアクセスが速くなります。  
   - データベースに格納する場合には、テーブルの主キーとして行IDカラムを設定し、インデックスを作成しておくことで、ID検索は高速なO(1)あるいはO(log n)アクセスを実現できます。

3. **検索の用途**:
   - 「指定した行ID（または範囲）を素早く取得したい」というニーズがあるなら、IDを割り振りインデックスすることは非常に有効です。  
   - テキストコンテンツ内の部分文字列検索を主要な目的とする場合は、テキスト全文に対する効率的な検索手法（インバーテッドインデックス、トライ構造、N-gramインデックスなど）の整備が必要で、行ID割り当てはあくまで補助的な役割になります。

**結論**:  
カンマ区切りごとに行IDを割り振ることで、「行単位での直接アクセス」や「ID指定での高速参照」などには効果が期待できます。しかし、テキスト内容に対する部分一致・全文検索を高速化したい場合には、より高度な検索インデックス（全文検索インデックス）を利用することが望ましく、行ID付与のみでは大幅な検索効率改善にはつながりにくいです。

#### Claud

データの性質と要件によりますが、行数のIDを割り振ることには以下のようなメリット・デメリットがあります：

メリット:

1. データの一意性が保証され、特定の行を参照しやすくなります
2. 行の追加・削除・並び替えの際にも一貫した参照が可能です
3. データベースでインデックスとして活用でき、検索速度が向上する可能性があります

デメリット:

1. データ構造が複雑になり、メンテナンスコストが増加します
2. ストレージ容量を余分に使用します
3. 単純なテキスト処理の場合、オーバーヘッドになる可能性があります

提示されたデータの場合：

- 行数が少なく、シンプルな構造です
- テキストベースの処理が中心と思われます

このケースでは、特別な要件（例：頻繁な検索や並び替え、他のデータとの結合）がない限り、行IDを追加するメリットは限定的かもしれません。通常の配列インデックスで十分対応できると考えられます。

検索性を本当に向上させたい場合は、どのような検索パターンが多いのかを教えていただけますか？それによって、より適切なインデックス設計を提案できると思います。

## 2023/12/29

<https://roadmap.sh/rust>
↑このサイトのアイデアをパクりたい

## 2024/12/30

リフレッシュボタン押下で再度データフェッチする
-> できるだけDBコスト抑えたいので

## 2024/12/31

Redisはリアルタイム性に優れているのでリアルタイム通知などの実装に使える
-> しかし、大量のデータの更新には向いていない感じがする

共同編集者に編集通知とかを飛ばす様な実装が可能
-> 編集通知がある -> 編集内容を同期するべきかの判定を簡単に行える(余分な更新を減らせる)

例のデータ形式(Json)
{
  "editor": "ryotarofr"
  "memo_id": "1",
  "comment": "2024_12_31のメモ"
}

記事データの管理方法について質問がございます(返信はいつでもOKです)。
私の記憶では、記事データ保存時にマークダウンをHTMLにパースしてそのままPosgreに突っ込んでいた気がしていますが、
これは頻繁にデータを更新しないという前提の実装になりますでしょうか？
また、更新頻度が高いと、トランザクションが失敗する可能性はありますか？

### ドキュメント型のDBを使う理由

テキストの効率的に保存するため。
テキスト取得更新がパフォーマンスの大部分になるから。

## 2025/1/1

やるべきとこ整理

- FE/BE データフロー完成
- BEデータフローの部分のみを作成
- FE最低限のコマンド作る

TODO:

BE作る

テストデータで試す