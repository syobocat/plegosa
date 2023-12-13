# これはなに

Pleromaに移住した結果Misskeyのアンテナが恋しくなってしまったので作ったエゴサツールです。

Plegosaという名前ですがおそらくMastodonでも動きます。FriendicaでもFirefishでも動きます。たぶん。

# どう使うの

`.env`ファイルを作って以下の情報を書きこめばOKです。環境変数でも問題ありませんが衝突防止の点から`.env`が好ましいでしょう。  
`USER_*`でのユーザーの書式は`@hoge@example.tld`ではなく`hoge@example.tld`なので注意してください。なお、ローカルのユーザーの場合は`@example.tld`すら不要で`hoge`のみです。

```
SOFTWARE=ソフトウェア名(例:Pleroma)
INSTANCE_URL=インスタンスのURL(例:pleroma.social)
ACCESS_TOKEN=アクセストークン(わからなければ空行にしておくと生成してくれます、設定は手動)
LOGGER=ヒットした投稿の出力先(現状stdoutとDiscordにのみ対応)
LOGGER_URL=DiscordのWebhook URL(LOGGERがDiscordの場合のみ)
TIMELINES=監視対象にするタイムライン(Home、PublicまたはLocalから複数選択可)
CASE_SENSITIVE=大文字/小文字、ひらがな/カタカナを区別する(true/false、デフォルト:true)
USE_REGEX=有効時、INCLUDEとEXCLUDEは正規表現として扱われます(true/false、デフォルト:false)
INCLUDE=ヒットさせたい単語(カンマ区切り、空の場合全てにヒットします)
EXCLUDE=ヒットさせたくない単語(カンマ区切り)
USER_INCLUDE=ヒットさせたいユーザー(カンマ区切り、空の場合全ユーザーにヒットします)
USER_EXCLUDE=ヒットさせたくないユーザー(カンマ区切り、自分の投稿を除外したいときなど)
```

# Known issues
  
- ソースコードが汚い  
  ちょっとずつマシになっているようないないような…

- 設定ファイルの破壊的変更大杉  
  もうちょっとしたら安定します

Contributeお待ちしております。
