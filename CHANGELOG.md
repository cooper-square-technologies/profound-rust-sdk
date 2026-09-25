# Changelog

## [0.1.1](https://github.com/cooper-square-technologies/profound-rust-sdk/compare/v0.1.0...v0.1.1) (2026-09-25)


### ⚠ BREAKING CHANGES

* **api:** 3 breaking changes to the SDK surface.
    - Property `bot_provider_filter.value` type changed from `enum(openai | anthropic | chatgpt | …) | Array<enum(openai | anthropic | chatgpt | …)>` to `enum(openai | anthropic | chatgpt | …) | Array<enum(openai | anthropic | chatgpt | …)>`.
    - Property `otf_intent_shares_query.platforms` type changed from `Array<string>` to `Array<enum(chatgpt.com | gemini.google.com | perplexity.ai)>`.
    - Property `otf_volume_request.platforms` type changed from `Array<string>` to `Array<enum(chatgpt.com | gemini.google.com | perplexity.ai)>`.
* **api:** 3 breaking changes to the SDK surface.
    - Property `bot_provider_filter.value` type changed from `enum(openai | anthropic | chatgpt | …) | Array<enum(openai | anthropic | chatgpt | …)>` to `enum(openai | anthropic | chatgpt | …) | Array<enum(openai | anthropic | chatgpt | …)>`.
    - Property `sentiment_v2_query.metrics` type changed from `Array<enum(positive_sentiment | negative_sentiment | occurrence)> | null` to `Array<enum(positive_sentiment | negative_sentiment | occurrence | …)> | null`.
    - Property `app_routes_v2_answer_engine_insights_reports_sentiment_sort_spec.field` type changed from `enum(occurrence | positive_sentiment | negative_sentiment)` to `enum(occurrence | positive_sentiment | negative_sentiment | …)`.
* **api:** 4 breaking changes to the SDK surface.
    - Serialization or defaults of query param `order_by` on `organizations.categories.prompts` changed.
    - Serialization or defaults of query param `order_dir` on `organizations.categories.prompts` changed.
    - Serialization or defaults of query param `version` on `agents.retrieve` changed.
    - Serialization or defaults of query param `version` on `agents.retrieveGraph` changed.
* **api:** 54 breaking changes to the SDK surface.
    - Property `accuracy_breakdown_query.limit` type changed from `integer` to `integer`.
    - Property `accuracy_breakdown_query.offset` type changed from `integer` to `integer`.
    - Property `answers_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `answers_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `citations_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `citations_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `claim_citations_query.limit` type changed from `integer` to `integer`.
    - Property `claim_citations_query.offset` type changed from `integer` to `integer`.
    - Property `cluster_example_runs_query.limit` type changed from `integer` to `integer`.
    - Property `cluster_example_runs_query.offset` type changed from `integer` to `integer`.
    - Property `create_project_task_request.impact` type changed from `integer | null` to `integer | null`.
    - Property `cursor_pagination.limit` type changed from `integer` to `integer`.
    - Property `factcheck_claims_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `factcheck_claims_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `factcheck_scores_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `factcheck_scores_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `inaccuracy_drivers_query.limit` type changed from `integer` to `integer`.
    - Property `inaccurate_clusters_query.limit` type changed from `integer` to `integer`.
    - Property `inaccurate_clusters_query.offset` type changed from `integer` to `integer`.
    - Property `inaccurate_themes_query.limit` type changed from `integer` to `integer`.
    - Property `inaccurate_themes_query.offset` type changed from `integer` to `integer`.
    - Property `pagination.limit` type changed from `integer` to `integer`.
    - Property `pagination.offset` type changed from `integer` to `integer`.
    - Property `project.task_count` type changed from `integer` to `integer`.
    - Property `project.new_task_count` type changed from `integer` to `integer`.
    - Property `project.version_count` type changed from `integer` to `integer`.
    - Property `project_attachment.size_bytes` type changed from `integer` to `integer`.
    - Property `project_detail.task_count` type changed from `integer` to `integer`.
    - Property `project_detail.new_task_count` type changed from `integer` to `integer`.
    - Property `project_detail.version_count` type changed from `integer` to `integer`.
    - Property `project_list_item.task_count` type changed from `integer` to `integer`.
    - Property `project_list_item.new_task_count` type changed from `integer` to `integer`.
    - Property `project_task.impact` type changed from `integer | null` to `integer | null`.
    - Property `project_task_detail.impact` type changed from `integer | null` to `integer | null`.
    - Property `query_fanouts_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `query_fanouts_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `search_knowledge_base_request.top_k` type changed from `integer` to `integer`.
    - Property `sentiment_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `sentiment_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `shopping_brands_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `shopping_brands_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `shopping_merchants_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `shopping_merchants_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `shopping_products_v2_query.competitor_limit` type changed from `integer` to `integer`.
    - Property `shopping_products_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `shopping_products_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `shopping_trigger_rate_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `shopping_trigger_rate_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `top_inaccurate_claims_query.limit` type changed from `integer` to `integer`.
    - Property `update_project_task_request.impact` type changed from `integer | null` to `integer | null`.
    - Property `visibility_v2_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `visibility_v2_query.max_results` type changed from `integer | null` to `integer | null`.
    - Property `youtube_channels_query.limit` type changed from `integer | null` to `integer | null`.
    - Property `youtube_videos_query.limit` type changed from `integer | null` to `integer | null`.

### Features

* **api:** add operation promptVolumes.volume.onTheFly (+8 more changes) ([2b715a0](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/2b715a00988fa8b36d476cbaac378ae05cfe2688))
* **api:** update property bot_provider_filter.value (+11 more changes) ([fc34ca8](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/fc34ca894124a5fcdc26acadef76ef64721d92a3))
* **api:** update property bot_provider_filter.value (+21 more changes) ([9ee1600](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/9ee1600952d39f11fb6247bc284c8f76cd407861))
* **api:** update SDK surface (5 changes) ([02d0d0a](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/02d0d0a9a4b577bcc8a269c62ddfebddc59055e6))
* **api:** update SDK surface (55 changes) ([29c2089](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/29c2089349431aeab36f63866c3fd685abcb1908))


### Chores

* **api:** regenerate SDK ([01beceb](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/01beceb2bd802de6c298e4835acee989be9b7218))
* release 0.1.1 ([729d5f4](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/729d5f4895f46b16e23f2fd2c86676fad5ca0ece))
* release 0.1.1 ([7300c85](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/7300c8531c187e82482f2a7996715fee0f5224b0))

## [0.1.0](https://github.com/cooper-square-technologies/profound-rust-sdk/compare/v0.0.1...v0.1.0) (2026-08-28)


### Features

* **api:** initial SDK generation ([cda7777](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/cda777735e063e1cc7a042f8e88a3386c33398c2))


### Chores

* **api:** regenerate SDK ([3f8abbb](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/3f8abbb3e7c7d5f312f5069398a995ceac079521))
* **api:** update generated SDK content ([58cfb76](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/58cfb760b169648388ab2d04453a451bf68c39d5))
* set version to 0.0.1 ([5344cf5](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/5344cf5b546ed5e1812c33688c1dc9cb7f195000))
* set version to 0.0.1 so the first release is 0.1.0 ([8abbcbe](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/8abbcbe3bff1da606dbf3ed36fdd8cb39f4038ed))

## Changelog

All notable changes to `profound` are documented here. Release
tooling appends a section per released version below.

## Unreleased

- Initial generation of the `profound` SDK.
- Response-only models are marked `#[non_exhaustive]`, so new response
  fields can be added in future versions without a breaking release;
  request models stay literally constructible.
