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
EXTRA_TIMELINE=追加で監視対象にするタイムライン(PublicまたはLocal)
CASE_SENSITIVE=大文字/小文字、ひらがな/カタカナを区別する(true/false、デフォルト:true)
INCLUDE=ヒットさせたい単語(カンマ区切り、空の場合全てにヒットします)
EXCLUDE=ヒットさせたくない単語(カンマ区切り)
USER_INCLUDE=ヒットさせたいユーザー(カンマ区切り、空の場合全ユーザーにヒットします)
USER_EXCLUDE=ヒットさせたくないユーザー(カンマ区切り、自分の投稿を除外したいときなど)
```

`EXTRA_TIMELINE`の挙動について

1. 未設定の場合、HTLのみを監視します。

2. Publicが指定されている場合、HTLに加えてGTL(「すべてのネットワーク」(Pleroma)、「連合タイムライン」(Mastodon))も監視します。(GTLが使用できるサーバーのみ)

3. Localが指定されている場合、HTLに加えてLTL(「公開タイムライン」(Pleroma))も監視します。(LTLが使用できるサーバーのみ)


# Known issues

- リピート(ブースト)のユーザー表記がおかしい  
  対処法がわからなかったので放置
  
- ソースコードが汚い  
  はい…

Contributeお待ちしております。
