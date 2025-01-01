文法ルールエンジン
IMEは単に単語を変換するだけでなく、日本語の文法規則を理解し、適切な変換候補を提供します。

文節分割:
IMEは、入力された文字列を文節（意味のある単位）に分割します。
例: わたしはにほんにいきます -> 私 | は | 日本 | に | 行きます
最適な組み合わせの推測:
各文節に対して複数の候補を生成し、統計的なモデル（例: Hidden Markov Model や深層学習）を使って最適な組み合わせを推測します。

## 場所

ターミナル直下で以下コマンド

```bash
cd ~/Library/Dictionaries/JapaneseInputMethod/
```

sqlite使う

```bash
sqlite3 ~/Library/Dictionaries/JapaneseInputMethod/DynamicPhraseLexicon_ja_JP.db
```

テーブル確認

```bash
sqlite> .table
```

バイナリにエンコードされているからわからん

```bash
sqlite> SELECT * FROM Words LIMIT 10;
1|501|0|0|1|1|L
2|678|n0|n0|1|1|?
3|640|0|0|1|1|G
4|319|k0|k0|1|1|?
5|361|g0|g0|1|1|
6|686|?|?|1|1|G
7|476|?0|?0|1|1|?
8|562|?0|?0|1|1|
9|497|W0f0|W0f0|3|3|??
10|655|g0|g0|1|1|?
```

## テーブル

Wordsユーザが動的に学習させたデータを格納

### カラム

sqlite> PRAGMA table_info(Words);
0|Identifier|INTEGER|0||1
1|Seed|INTEGER|0||0
2|Reading|BLOB|0||0
3|Surface|BLOB|0||0
4|ReadingSegments|INTEGER|0||0
5|SurfaceSegments|INTEGER|0||0
6|PartOfSpeech|BLOB|0||0

## **テーブル構造の詳細**

| カラム番号 | カラム名         | データ型   | NOT NULL | デフォルト値 | 主キー |
|------------|------------------|------------|----------|--------------|--------|
| 0          | `Identifier`    | `INTEGER`  | 0        |              | 1      |
| 1          | `Seed`          | `INTEGER`  | 0        |              | 0      |
| 2          | `Reading`       | `BLOB`     | 0        |              | 0      |
| 3          | `Surface`       | `BLOB`     | 0        |              | 0      |
| 4          | `ReadingSegments` | `INTEGER` | 0        |              | 0      |
| 5          | `SurfaceSegments` | `INTEGER` | 0        |              | 0      |
| 6          | `PartOfSpeech`  | `BLOB`     | 0        |              | 0      |

---

## **カラムの役割**

### **1. `Identifier`**

- **型**: `INTEGER`
- **役割**: 各レコードを一意に識別する主キー。

### **2. `Seed`**

- **型**: `INTEGER`
- **役割**: 単語やフレーズのスコアや頻度を表す値。
  - よく使われる単語やフレーズほど高い値になる可能性。

### **3. `Reading`**

- **型**: `BLOB`
- **役割**: 読み仮名を表すデータ（エンコード済み）。
  - 例: `ありがとう`, `おはよう`。

### **4. `Surface`**

- **型**: `BLOB`
- **役割**: 変換後の文字列を表すデータ（エンコード済み）。
  - 例: `有難う`, `お早う`。

### **5. `ReadingSegments`**

- **型**: `INTEGER`
- **役割**: 読み仮名を分割した文節の数。

### **6. `SurfaceSegments`**

- **型**: `INTEGER`
- **役割**: 変換結果（漢字など）を分割した文節の数。

### **7. `PartOfSpeech`**

- **型**: `BLOB`
- **役割**: 単語やフレーズの品詞情報。
  - 例: 「名詞」「動詞」「助詞」など。

---

この形式で保存すると、ドキュメント化や共有に便利です！さらに内容を追加したい場合はお知らせください！
