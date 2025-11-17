<!--
SPDX-FileCopyrightText: 2023-2025 SyoBoN <syobon@syobon.net>

SPDX-License-Identifier: CC-BY-4.0
-->

# v0.4

## 変更

- `config.toml`の`url`にスキーマが必須に (例: `example.tld` → `https://example.tld`)
- `timelines.home`、`timelines.local`、`timelines.public`を削除し、`timeline.targets`に
- `openssl`が(たぶん)不要に

## 追加

- illumos向けビルドを追加
- `PLEGOSA_CONFIG`環境変数で設定ファイルのパスを変更できるように

# v0.3

## 修正

- 引用を無視しないように

# v0.2.3

## 追加

- Unicode正規化に対応

## 削除

- Solaris 10向けビルドを廃止 (Solaris 11向けビルドは現在利用できません)

# v0.2.2

## 修正

- `include`が正常に動作するように

# v0.2.1

## 修正

- `exclude`が正常に動作するように

# v0.2.0

## 変更

- .envからconfig.tomlに移行
- Discord loggerがデフォルトでEmbedを使用するように

## 追加

- 複数のloggerに対応
- Discord loggerに常にEmbedを使用するオプションを追加
- AArch64環境のLinux向けビルドを追加

## 修正

- リピートが記録されないように

# v0.1.2

## 追加

- Solaris向けビルドを追加
- OpenSSLを静的にリンクするFeature Flagを追加

# v0.1.1

## 追加

- NetBSD向けビルドを追加
- linux-muslとNetBSDを除きOpenSSLを動的にリンクするように

# v0.1.0

リリース
