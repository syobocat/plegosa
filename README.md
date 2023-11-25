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
INCLUDE=ヒットさせたい単語(カンマ区切り、空の場合全てにヒットします)
EXCLUDE=ヒットさせたくない単語(カンマ区切り)
USER_INCLUDE=ヒットさせたいユーザー(カンマ区切り、空の場合全ユーザーにヒットします)
USER_EXCLUDE=ヒットさせたくないユーザー(カンマ区切り、自分の投稿を除外したいときなど)
```

`ACCESS_TOKEN`は状態によって挙動が異なります。

1. 設定されていない場合  
アクセストークンの生成処理が走ります。

2. 設定されているが空の場合(`ACCESS_TOKEN=`の状態)  
監視対象がGTL(「すべてのネットワーク」(Pleroma)、「連合タイムライン」(Mastodon))になります。

3. 設定されていて、中身が存在する場合(`ACCESS_TOKEN=xxxxxxxxxx`の状態)  
監視対象がHTLになります。

# Known issues

- リピート(ブースト)のユーザー表記がおかしい  
  対処法がわからなかったので放置
  
- ソースコードが汚い  
  はい…

Contributeお待ちしております。
