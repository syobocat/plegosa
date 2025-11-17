<!--
SPDX-FileCopyrightText: 2023-2025 SyoBoN <syobon@syobon.net>

SPDX-License-Identifier: CC-BY-4.0
-->

# これはなに

Pleromaに移住した結果Misskeyのアンテナが恋しくなってしまったので作ったエゴサツールです。

Plegosaという名前ですがおそらくMastodonでも動きます。FriendicaでもFirefishでも動きます。たぶん。

# どう使うの

`config.toml`ファイルを作って以下の情報を書きこめばOKです。  
`user_*`でのユーザーの書式は`@hoge@example.tld`ではなく`hoge@example.tld`なので注意してください。なお、ローカルのユーザーの場合は`@example.tld`すら不要で`hoge`のみです。

<!--
SPDX-SnippetBegin
SPDX-SnippetCopyrightText: NONE

SPDX-License-Identifier: CC0-1.0
-->
```toml
[instance]
software = 'Pleroma' # ソフトウェア名
url = 'https://pleroma.social' # インスタンスのURL
token = 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx' # アクセストークン(timelines.targetsに'Home'が含まれる場合のみ必須、わからなければ空にしておくと生成してくれます、設定は手動)

[timeline]
targets = [ 'Home' ] # 監視するタイムライン ('Home'、'Local'、'Public'が指定可能)

[filter]
case_sensitive = true # include, excludeで大文字/小文字、ひらがな/カタカナ及び互換等価な字を区別するかどうか(trueでも正準等価な字は区別しません) (デフォルト: true)
use_regex = false # include, excludeを正規表現として扱うかどうか (デフォルト: false)
include = [] # ヒットさせたい単語(空の場合全てにヒットします)
exclude = [] # 除外したい単語
user_include = [] # ヒットさせたいユーザー(空の場合全ユーザーにヒットします)
user_exclude = [] # ヒットさせたくないユーザー(自分の投稿を除外したいときなど)

[logger.stdout]
enable = true # ヒットログを標準出力に書き込むかどうか (デフォルト: true)

[logger.discord]
enable = false # ヒットログをDiscordに送信するかどうか (デフォルト: false)
use_embed = true # リンクを直貼りするのではなくEmbedとして送信する(公開範囲によってはfalseでもEmbedとして送信されます) (デフォルト: true)
webhook = 'https://discord.com/api/webhooks/0000000000000000000/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx-xxxxxxxx-xxxxxxxxxxxxxxxxxx' # WebhookのURL
```
<!--SPDX-SnippetEnd-->
