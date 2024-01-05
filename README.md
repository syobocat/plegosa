# これはなに

Pleromaに移住した結果Misskeyのアンテナが恋しくなってしまったので作ったエゴサツールです。

Plegosaという名前ですがおそらくMastodonでも動きます。FriendicaでもFirefishでも動きます。たぶん。

# 依存関係を教えて

- ビルド時依存
  - rust
  - gmake (`static-openssl`が有効の場合)
  - perl (`static-openssl`が有効の場合)
- 実行時依存
  - openssl (`static-openssl`が無効の場合)

たぶんこれで全部。

# どう使うの

`config.toml`ファイルを作って以下の情報を書きこめばOKです。  
`user_*`でのユーザーの書式は`@hoge@example.tld`ではなく`hoge@example.tld`なので注意してください。なお、ローカルのユーザーの場合は`@example.tld`すら不要で`hoge`のみです。

```toml
[instance]
software = 'Pleroma' # ソフトウェア名
url = 'pleroma.social' # インスタンスのURL
token = 'xxxxxxxxxxxxxxxxxxxx_xxxxxxxxxxxxxxxxxxxxxx' # アクセストークン(timelines.homeがtrueの時のみ必須、わからなければ空にしておくと生成してくれます、設定は手動)

[timelines]
home = true    # ホームタイムラインを監視するかどうか (デフォルト: true)
local = false  # ローカルタイムラインを監視するかどうか (デフォルト: false)
public = false # グローバルタイムラインを監視するかどうか (デフォルト: false)

[filter]
case_sensitive = true # include, excludeで大文字/小文字、ひらがな/カタカナを区別するかどうか (デフォルト: true)
use_regex = false # include, excludeを正規表現として扱うかどうか (デフォルト: false)
include = [] # ヒットさせたい単語(空の場合全てにヒットします)
exclude = [] # 除外したい単語
user_include = [] # ヒットさせたいユーザー(空の場合全ユーザーにヒットします)
user_exclude = [] # ヒットさせたくないユーザー(自分の投稿を除外したいときなど)

[logger.stdout]
enable = true # ヒットログを標準出力に書き込むかどうか (デフォルト: true)

[logger.discord]
enable = false # ヒットログをDiscordに送信するかどうか (デフォルト: false)
use_embed = false # リンクを直貼りするのではなくEmbedとして送信する(公開範囲によってはfalseでもEmbedとして送信されます) (デフォルト: false)
webhook = 'https://discord.com/api/webhooks/0000000000000000000/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx-xxxxxxxx-xxxxxxxxxxxxxxxxxx' # WebhookのURL
```

# Known issues
  
- ソースコードが汚い  
  ちょっとずつマシになっているようないないような…

- 設定ファイルの破壊的変更大杉  
  もうちょっとしたら安定します

Contributeお待ちしております。
