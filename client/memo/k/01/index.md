# divタグでメモ帳を作る

メモ管理共有ツールが欲しいと思っていました。考えていたものを紹介します。

## 欲しい機能

- マークダウンが使える
- 図式を描画できる
- クラウドで管理できる(webで使える)
- 簡単に共有できる

## divを使う理由

世代管理したかったから。独自カスタマイズをしたい。

ここでの世代管理はgitみたいな仕組みを指しています。更新日時と更新ユーザの情報を「コミット」という単位で管理したいと考えています。

ちなみに、今のgoogleスプレットシートはdivタグでinputフォームを実装しています。
(確か1年前はcanvasを使っていたと思います。)

## 作成中のもの

[動画](https://youtu.be/cPqs2mZ5loE)

## 現段階で実装した機能

- カーソル
- 字句解析器
  - 入力キー解析器
    - IME入力
  - マークダウン解析器
  - カスタムコンポーネント解析器
  - 組み合わせキー(ショートカット等)
- ドラック機能
  - コピー&ペースト
  - 「シフト + 矢印キー」
  - 列・行移動機能

## ソース

[mijinko](https://github.com/ryotarofr/_mijinko)

## 追加で実装したい機能

- プラグイン
  - 絵文字ツール
  - 自動リンクツール
  - ツールバー
  - ツリービュー
  - コードハイライト
  - Autocomplete
    - 予測変換の最適化
- 時系列管理ツール(データ更新するために必要)
  - 更新ユーザー管理
  - 更新時系列管理
  - キャッシュ管理

## 技術

- Dioxus(FE)
- Axum(BE)
- Redis(キャッシュ管理)

世代管理の仕組みは[diamond-types](https://github.com/ryotarofr/diamond-types)で多分できます。
ただ、できるだけ軽量のテキストティタにしたいので[CRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)の不要な部分を取り除く必要はありそうです。

データフローについては、divで実装することでVMNodeのJsonをそのままRedisに投げれそうなので問題ないはずです。

divでの実装は自由にカスタマイズできる反面、手動で多くの機能を実装する必要があります。例えば、テキストエリアのheightが100vhを超えた場合にカーソルに追従して自動スクロールさせる必要があります。

webの高度なイベント制御などを知る機会として有意義なものになりそうです。
