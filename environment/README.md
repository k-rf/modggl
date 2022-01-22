# Pulumi を使った構成管理

## 本番環境を構築する

```bash
yarn up:production
```

## ステージング環境を構築する

```bash
yarn up:staging
```

## パイプラインを構築する

```bash
yarn up:pipeline
```

## リソースを削除する

```bash
pulumi destroy -s poiulkjhmnbv/modggl/staging -C src/deployment
pulumi destroy -s poiulkjhmnbv/modggl/production -C src/deployment
pulumi destroy -s poiulkjhmnbv/modggl-pipeline/pipeline -C src/pipeline
```

## スタックを削除する

```bash
pulumi stack rm production -C src/deployment
pulumi stack rm staging -C src/deployment
pulumi stack rm pipeline -C src/pipeline
```

## GCP の認証を通す

1 回だけ実行すればよい。
`starship` を利用していれば、認証されているとターミナルに雲のアイコンが表示される。

```bash
gcloud config set project <YOUR_GCP_PROJECT_ID>
gcloud auth application-default login
```
