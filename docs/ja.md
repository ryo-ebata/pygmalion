# Pygmalion

Infrastructure as Code（IaC）定義から自動的にインフラ構成図を生成する、高性能で開発者中心の拡張可能なコマンドラインインターフェース（CLI）ツールです。

## 🎯 ミッション

Pygmalionは、堅牢な共通中間表現（IR）を介して、あらゆるIaC定義をあらゆる図形描画フォーマットに変換することで、インフラの可視化を統一します。私たちのミッションは、Terraform、AWS CDK、Pulumiといった複数のIaCツールを使用しながら統一された可視化・文書化ツールを欠いている現代のDevOps環境における断片化を解決することです。

## 🏗️ アーキテクチャ

Pygmalionは、プラグインベースエコシステムを備えた3ステージパイプラインアーキテクチャに従います：

```
パース → 中間表現 → レンダリング
```

### コアコンポーネント

1. **パースステージ**: IaC固有のパーサープラグインがソースコードを標準化されたIR JSONオブジェクトに変換
2. **中間表現（IR）**: 完全なインフラグラフを表現する宣言的でシリアライズ可能なJSONフォーマット
3. **レンダリングステージ**: レンダラープラグインがIRを視覚的出力（Draw.io、Mermaid.jsなど）に変換

### プラグインエコシステム

- **言語非依存**: プラグインはPATH経由で発見されるスタンドアロン実行可能ファイル
- **標準IPC**: JSONによるstdin/stdout通信
- **拡張可能**: プラグインを通じてあらゆるIaCツールや図形描画フォーマットをサポート

## 🚀 機能

### 現在（MVP）
- **マルチIaCサポート**: Terraform、AWS CDK、Pulumi
- **マルチフォーマット出力**: Draw.io、Mermaid.js
- **プラグインアーキテクチャ**: 拡張可能なパーサーとレンダラーシステム
- **高性能**: Rustベースの実装
- **CI/CD対応**: 単一のネイティブバイナリ配布

### 計画中
- **コスト分析**: クラウド価格APIとの統合
- **運用シミュレーション**: 使用例とスクリプトの生成
- **ドキュメント自動生成**: 包括的なREADMEファイルの自動生成
- **セキュリティスキャン**: コンプライアンスとセキュリティ調査結果の可視化

## 📦 インストール

### 前提条件
- Rust 1.70+（開発用）
- 可視化するTerraform、AWS CDK、またはPulumiプロジェクト

### クイックスタート
```bash
# リポジトリをクローン
git clone https://github.com/your-org/pygmalion.git
cd pygmalion

# ソースからビルド
cargo build --release

# グローバルにインストール
cargo install --path .
```

## 🛠️ 使用方法

### 基本コマンド

```bash
# Terraformプロジェクトから図を生成
pygma generate ./terraform-project --from terraform --to drawio

# CDKプロジェクトからMermaid図を生成
pygma generate ./cdk-project --from cdk --to mermaid -o architecture.md

# パースのみ（IR JSONを出力）
pygma parse ./terraform-project --from terraform > infra.ir.json

# レンダリングのみ（IR JSONから）
pygma render infra.ir.json --to drawio -o diagram.drawio.xml

# 利用可能なプラグインを一覧表示
pygma plugin list
```

### コマンドリファレンス

| コマンド | 引数 | 説明 |
|---------|-----------|-------------|
| `generate <PATH>` | `--from <FORMAT>`, `--to <FORMAT>`, `-o <FILE>` | 1つのコマンドでパースとレンダリングを実行 |
| `parse <PATH>` | `--from <FORMAT>`, `-o <FILE>` | IaCをIR JSONにパース |
| `render <IR_FILE>` | `--to <FORMAT>`, `-o <FILE>` | IRを図形フォーマットにレンダリング |
| `plugin list` | なし | 発見されたプラグインを一覧表示 |

### サポートされているフォーマット

**入力IaCフォーマット:**
- `terraform` - Terraform HCLファイル
- `cdk` - AWS CDKプロジェクト（TypeScript、Pythonなど）
- `pulumi` - Pulumiプロジェクト

**出力図形フォーマット:**
- `drawio` - Draw.io互換XML
- `mermaid` - Mermaid.js構文

## 🔌 プラグイン開発

Pygmalionのプラグインアーキテクチャにより、あらゆるIaCツールや図形描画フォーマットのサポートを拡張できます。

### プラグイン発見
プラグインは命名規則に従うスタンドアロン実行可能ファイルです：
- パーサー: `pygma-parser-<format>`
- レンダラー: `pygma-renderer-<format>`

### プラグインインターフェース

**パーサープラグイン契約:**
```bash
# 入力: コマンドライン引数
pygma-parser-terraform --input-path ./project --output-path ./output

# 出力: IR JSONをstdoutに
{
  "irVersion": "1.0.0",
  "nodes": [...],
  "edges": [...],
  "metadata": {...}
}
```

**レンダラープラグイン契約:**
```bash
# 入力: stdinからIR JSON
cat infra.ir.json | pygma-renderer-drawio --output diagram.drawio.xml

# 出力: 指定されたパスまたはstdoutに図形ファイル
```

### IRスキーマ

中間表現（IR）は標準化されたJSONフォーマットです：

```json
{
  "irVersion": "1.0.0",
  "metadata": {
    "sourceType": "terraform"
  },
  "nodes": [
    {
      "id": "vpc-123",
      "type": "Resource",
      "label": "プライマリVPC",
      "properties": {
        "instance_type": "t3.micro"
      },
      "metadata": {
        "cost": {
          "monthly": 7.59
        }
      },
      "parent": "subnet-abc",
      "icon": "aws-ec2-instance"
    }
  ],
  "edges": [
    {
      "id": "dep-vpc-subnet",
      "source": "vpc-123",
      "target": "subnet-abc",
      "type": "Dependency",
      "label": "依存関係"
    }
  ]
}
```

## 🏛️ アーキテクチャ詳細

### 3ステージパイプライン

1. **パースステージ**: IaC固有のパーサーがインフラ情報を抽出
2. **IRステージ**: リッチなメタデータを持つ統一グラフモデル
3. **レンダリングステージ**: レイアウトエンジンと視覚的フォーマット

### レイアウトエンジン

Pygmalionは最適な図形レイアウトのために杉山（Sugiyama）メソッド（階層的グラフ描画）を使用します：
- エッジの交差を最小化
- 明確な依存関係フローを作成
- 論理的なグルーピングとクラスタリングをサポート

### パフォーマンス最適化

- **Rust実装**: メモリ安全性と高性能
- **単一バイナリ**: 簡単な配布とCI/CD統合
- **ストリーミング処理**: 大規模なインフラグラフを効率的に処理
- **キャッシュ**: インクリメンタル更新のためのIRキャッシュ

## 🛣️ ロードマップ

### フェーズ1: コアプラットフォーム（現在）
- [x] 基本的なCLIフレームワーク
- [ ] IRスキーマ実装
- [ ] プラグインシステム
- [ ] Terraformパーサー
- [ ] Draw.ioレンダラー
- [ ] Mermaidレンダラー

### フェーズ2: 拡張機能
- [ ] コスト分析エンジン
- [ ] 運用シミュレーション
- [ ] ドキュメント自動生成
- [ ] セキュリティスキャン統合

### フェーズ3: 高度な機能
- [ ] リアルタイム監視統合
- [ ] マルチクラウドサポート
- [ ] 高度なレイアウトアルゴリズム
- [ ] インタラクティブWebインターフェース

## 🤝 貢献

貢献を歓迎します！詳細については[貢献ガイド](CONTRIBUTING.md)をご覧ください。

### 開発環境セットアップ

```bash
# クローンとセットアップ
git clone https://github.com/your-org/pygmalion.git
cd pygmalion

# 依存関係をインストール
cargo build

# テストを実行
cargo test

# リンターを実行
cargo clippy

# コードをフォーマット
cargo fmt
```

### プラグイン開発

1. 命名規則に従って新しい実行可能ファイルを作成
2. プラグイン契約を実装（stdin/stdout JSON通信）
3. テストとドキュメントを追加
4. プルリクエストを提出

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。

## 🙏 謝辞

- AWS PDKの[cdk-graph](https://github.com/awslabs/aws-pdk/tree/mainline/ws/cdk-graph)にインスパイアされました
- CLIフレームワークとして[clap](https://github.com/clap-rs/clap)を使用
- レイアウトアルゴリズムに[Graphviz](https://graphviz.org/)を使用
- アイコンは[AWS Architecture Icons](https://aws.amazon.com/architecture/icons/)から

## 📞 サポート

- **Issues**: [GitHub Issues](https://github.com/your-org/pygmalion/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/pygmalion/discussions)
- **Documentation**: [Wiki](https://github.com/your-org/pygmalion/wiki)

---

**Pygmalion**: Infrastructure as Codeを美しく意味のある図形に変換します。
