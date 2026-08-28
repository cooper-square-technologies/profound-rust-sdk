# profound Rust API

Complete reference of every operation, grouped by resource. See [the README](./README.md) for usage and configuration.

## Contents

- [`Organizations`](#organizations)
  - [Get Regions](#get-regions)
  - [Get Models](#get-models)
  - [Get Domains](#get-domains)
  - [Get Assets](#get-assets)
  - [Get Personas](#get-personas)
  - [List organizations](#list-organizations)
  - [`Organizations Categories`](#organizations-categories)
    - [Get Categories](#get-categories)
    - [Get Category Topics](#get-category-topics)
    - [Get Category Tags](#get-category-tags)
    - [List prompts](#list-prompts)
    - [Get Category Assets](#get-category-assets)
    - [Get Category Personas](#get-category-personas)
    - [Create prompts](#create-prompts)
    - [Update prompts](#update-prompts)
    - [Update prompt status](#update-prompt-status)
    - [Get Category Regions](#get-category-regions)
    - [Get Category Citation Categories](#get-category-citation-categories)
- [`Prompts`](#prompts)
  - [Get Answers](#get-answers)
  - [Query Answers V2](#query-answers-v2)
  - [Stream Answers V2](#stream-answers-v2)
- [`Reports`](#reports)
  - [Query Citations](#query-citations)
  - [Query Visibility](#query-visibility)
  - [Query Sentiment](#query-sentiment)
  - [Query Sentiment V2](#query-sentiment-v2)
  - [Get Referrals Report V1](#get-referrals-report-v1)
  - [Get Bots Report V1](#get-bots-report-v1)
  - [Query Fanouts](#query-fanouts)
  - [Stream Citations](#stream-citations)
  - [Stream Visibility](#stream-visibility)
  - [Stream Sentiment](#stream-sentiment)
  - [Stream Citations V2](#stream-citations-v2)
  - [Stream Visibility V2](#stream-visibility-v2)
  - [Stream Sentiment V2](#stream-sentiment-v2)
  - [Stream Query Fanouts V2](#stream-query-fanouts-v2)
  - [Get Referrals Report V2](#get-referrals-report-v2)
  - [Get Bots Report V2](#get-bots-report-v2)
  - [Query Visibility V2](#query-visibility-v2)
  - [Query Citations V2](#query-citations-v2)
  - [Query Sentiment V2](#query-sentiment-v2-1)
  - [Query Fanouts V2](#query-fanouts-v2)
  - [`Reports WebSearchResults`](#reports-websearchresults)
    - [Query Web Search Results](#query-web-search-results)
    - [Stream Web Search Results](#stream-web-search-results)
  - [`Reports Shopping`](#reports-shopping)
    - [Query Shopping Brands V2](#query-shopping-brands-v2)
    - [Stream Shopping Brands V2](#stream-shopping-brands-v2)
    - [Query Shopping Products V2](#query-shopping-products-v2)
    - [Stream Shopping Products V2](#stream-shopping-products-v2)
    - [Query Shopping Merchants V2](#query-shopping-merchants-v2)
    - [Stream Shopping Merchants V2](#stream-shopping-merchants-v2)
    - [Query Shopping Trigger Rate V2](#query-shopping-trigger-rate-v2)
    - [Stream Shopping Trigger Rate V2](#stream-shopping-trigger-rate-v2)
  - [`Reports Accuracy`](#reports-accuracy)
    - [Accuracy Overview](#accuracy-overview)
    - [Accuracy Breakdown](#accuracy-breakdown)
    - [Accuracy Citation Analysis](#accuracy-citation-analysis)
    - [Accuracy Topic Ids](#accuracy-topic-ids)
    - [Accuracy Inaccurate Themes](#accuracy-inaccurate-themes)
    - [Accuracy Inaccurate Clusters](#accuracy-inaccurate-clusters)
    - [Accuracy Inaccuracy Drivers](#accuracy-inaccuracy-drivers)
    - [Accuracy Top Inaccurate Claims](#accuracy-top-inaccurate-claims)
    - [Accuracy Claim Breakdown](#accuracy-claim-breakdown)
    - [Accuracy Claim Citations](#accuracy-claim-citations)
    - [Accuracy Cluster Example Runs](#accuracy-cluster-example-runs)
    - [Accuracy Cluster Verification Pairs](#accuracy-cluster-verification-pairs)
    - [Accuracy Factcheck Setup Status](#accuracy-factcheck-setup-status)
  - [`Reports Factcheck`](#reports-factcheck)
    - [Query Scores](#query-scores)
    - [Stream Scores](#stream-scores)
    - [`Reports Factcheck Claims`](#reports-factcheck-claims)
      - [Query Claims](#query-claims)
      - [Stream Claims](#stream-claims)
  - [`Reports Social`](#reports-social)
    - [`Reports Social Youtube`](#reports-social-youtube)
      - [Query Youtube Channels](#query-youtube-channels)
      - [Query Youtube Videos](#query-youtube-videos)
      - [Query Youtube Summary](#query-youtube-summary)
- [`Content`](#content)
  - [`Content Optimization`](#content-optimization)
    - [Optimization List](#optimization-list)
    - [Optimization Analysis](#optimization-analysis)
- [`Agents`](#agents)
  - [List agents](#list-agents)
  - [Get an agent](#get-an-agent)
  - [Create an agent](#create-an-agent)
  - [Publish an agent](#publish-an-agent)
  - [Update an agent](#update-an-agent)
  - [Get an agent's graph](#get-an-agents-graph)
  - [`Agents Runs`](#agents-runs)
    - [Run an agent](#run-an-agent)
    - [Get an agent run](#get-an-agent-run)
  - [`Agents NodeTypes`](#agents-nodetypes)
    - [List node types](#list-node-types)
    - [Get a node type schema](#get-a-node-type-schema)
- [`KnowledgeBases`](#knowledgebases)
  - [List Knowledge Bases](#list-knowledge-bases)
  - [Search Knowledge Base](#search-knowledge-base)
  - [`KnowledgeBases Documents`](#knowledgebases-documents)
    - [Add Document](#add-document)
    - [Update Document](#update-document)
    - [Delete Document](#delete-document)
  - [`KnowledgeBases Folders`](#knowledgebases-folders)
    - [Add Folder](#add-folder)
    - [Delete Folder](#delete-folder)
- [`Projects`](#projects)
  - [List Projects](#list-projects)
  - [Create Project](#create-project)
  - [Get Project](#get-project)
  - [Delete Project](#delete-project)
  - [Get Project Status](#get-project-status)
  - [Archive Project](#archive-project)
  - [Unarchive Project](#unarchive-project)
  - [`Projects Generations`](#projects-generations)
    - [List Project Generations](#list-project-generations)
    - [Get Project Generation Status](#get-project-generation-status)
  - [`Projects Tasks`](#projects-tasks)
    - [List Project Tasks](#list-project-tasks)
    - [Create Project Task](#create-project-task)
    - [Get Project Task](#get-project-task)
    - [Update Project Task](#update-project-task)
    - [Delete Project Task](#delete-project-task)
    - [Update Project Task Status](#update-project-task-status)
- [`Integrations`](#integrations)
  - [List Integrations](#list-integrations)
- [`Documents`](#documents)
  - [Create a document](#create-a-document)
  - [List documents](#list-documents)
  - [Read a document](#read-a-document)
  - [Rename or reshare a document](#rename-or-reshare-a-document)
  - [Delete a document](#delete-a-document)
  - [Replace a document's content](#replace-a-documents-content)
- [`Ads`](#ads)
  - [`Ads OpenaiAds`](#ads-openaiads)
    - [`Ads OpenaiAds AdAccount`](#ads-openaiads-adaccount)
      - [Get Account Insights](#get-account-insights)

## Setup

```rust
use profound::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ProfoundClient::from_env()?;

    // ... the samples below go here

    Ok(())
}
```

## `Organizations`

### Get Regions

Get the organization regions.

| Direction | Type |
| --- | --- |
| Response | [`Vec<NamedResource>`](./src/models/organizations.rs) |

```rust
let response = client.organizations().regions().send().await?;
```

### Get Models

Get the organization models.

| Direction | Type |
| --- | --- |
| Response | [`Vec<NamedResource>`](./src/models/organizations.rs) |

```rust
let response = client.organizations().models().send().await?;
```

### Get Domains

Get the organization domains.

| Direction | Type |
| --- | --- |
| Response | [`Vec<DomainWithOrganization>`](./src/models/organizations.rs) |

```rust
let response = client.organizations().domains().send().await?;
```

### Get Assets

Get the organization assets, one row per (asset, organization) pair.

An asset's category can belong to multiple organizations; one asset row is
emitted per owning org so no association is silently dropped.

| Direction | Type |
| --- | --- |
| Response | [`OrganizationAssetsResponse`](./src/models/organizations.rs) |

```rust
let response = client.organizations().list_assets().send().await?;
```

### Get Personas

Get the organization personas, one row per (persona, organization) pair.

Same (item, org) fan-out as ``get_assets``: a persona's category can be
owned by multiple orgs, and each owning org gets its own row so no
association is silently dropped.

| Direction | Type |
| --- | --- |
| Response | [`OrganizationPersonasWithCategoryResponse`](./src/models/organizations.rs) |

```rust
let response = client.organizations().get_personas().send().await?;
```

### List organizations

Return every organization the caller's API key grants access to. Use this to discover organization IDs before calling endpoints that accept an `organization_id` filter.

| Direction | Type |
| --- | --- |
| Response | [`Vec<Organization>`](./src/models/organizations.rs) |

```rust
let response = client.organizations().list().send().await?;
```

### `Organizations Categories`

#### Get Categories

Get the organization categories, one row per (category, organization) pair.

| Direction | Type |
| --- | --- |
| Response | [`Vec<CategoryWithOrganization>`](./src/models/categories.rs) |

```rust
let response = client.organizations().categories().list().send().await?;
```

#### Get Category Topics

Get the topics for a specific category.

| Direction | Type |
| --- | --- |
| Response | [`Vec<Topic>`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .topics("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get Category Tags

Get the tags for a specific category.

| Direction | Type |
| --- | --- |
| Response | [`Vec<Tag>`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .tags("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### List prompts

Retrieve prompts in a category with optional filtering by type, topic, tag, region, platform, or persona. Supports cursor-based pagination.

| Direction | Type |
| --- | --- |
| Response | [`CategoryPromptsResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .prompts("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get Category Assets

| Direction | Type |
| --- | --- |
| Response | [`Vec<CategoryAsset>`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .assets("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get Category Personas

| Direction | Type |
| --- | --- |
| Response | [`CategoryPersonasResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .get_category_personas("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Create prompts

Create one or more prompts in a category. Topics and tags are auto-created if referenced by name and not yet existing. Use dry_run to preview without persisting.

| Direction | Type |
| --- | --- |
| Request | [`CreatePromptsBody`](./src/models/categories.rs) |
| Response | [`CreatePromptsResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .create_prompts(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        CreatePromptsBody {
            prompts: vec![CreatePromptInput {
                id: None,
                prompt: "x".to_string(),
                topic: IdOrName { id: None, name: None },
                language: "".to_string(),
                tags: None,
                regions: vec![IdOrName { id: None, name: None }],
                platforms: vec![IdOrName { id: None, name: None }],
                personas: None,
                analysis_types: None,
                prompt_type: None,
                asset: None,
            }],
            dry_run: None,
        },
    )
    .send()
    .await?;
```

#### Update prompts

Update one or more existing prompts. Only provided fields are changed. Dimension fields (regions, platforms, personas, tags) replace the full set when provided. Use dry_run to preview without persisting.

| Direction | Type |
| --- | --- |
| Request | [`UpdatePromptsBody`](./src/models/categories.rs) |
| Response | [`UpdatePromptsResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .update_prompts(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdatePromptsBody {
            prompts: vec![UpdatePromptInput {
                id: "".to_string(),
                prompt: None,
                topic: None,
                language: None,
                tags: None,
                regions: None,
                platforms: None,
                personas: None,
                analysis_types: None,
                prompt_type: None,
                asset: None,
            }],
            dry_run: None,
        },
    )
    .send()
    .await?;
```

#### Update prompt status

Bulk-update the status of one or more prompts. Prompts already in the target status are skipped. Use dry_run to preview without persisting.

Status options:
- 'active': Prompts will run daily.
- 'disabled': Prompts will not run moving forward, but historical data is preserved.
- 'deleted': Prompts are deleted along with historical data

| Direction | Type |
| --- | --- |
| Request | [`UpdatePromptStatusBody`](./src/models/categories.rs) |
| Response | [`UpdatePromptStatusResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .update_prompt_status(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdatePromptStatusBody {
            prompt_ids: vec!["7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string()],
            status: UpdatePromptStatusBodyStatus::Active,
            dry_run: None,
        },
    )
    .send()
    .await?;
```

#### Get Category Regions

Get the regions for a specific category.

| Direction | Type |
| --- | --- |
| Response | [`Vec<NamedResource>`](./src/models/organizations.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .retrieve_regions("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get Category Citation Categories

Get the citation categories for a category: the built-in buckets plus any custom categories.

| Direction | Type |
| --- | --- |
| Response | [`CitationCategoriesResponse`](./src/models/categories.rs) |

```rust
let response = client
    .organizations()
    .categories()
    .get_citation_categories("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Prompts`

### Get Answers

| Direction | Type |
| --- | --- |
| Request | [`AnswersQuery`](./src/models/prompts.rs) |
| Response | [`AnswersResponse`](./src/models/prompts.rs) |

```rust
let response = client
    .prompts()
    .answers(AnswersQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        pagination: None,
        filters: None,
        include: None,
    })
    .send()
    .await?;
```

### Query Answers V2

| Direction | Type |
| --- | --- |
| Request | [`AnswersV2Query`](./src/models/prompts.rs) |
| Response | [`AnswersV2Response`](./src/models/prompts.rs) |

```rust
let response = client
    .prompts()
    .answers_v2(AnswersV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        include: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

### Stream Answers V2

| Direction | Type |
| --- | --- |
| Request | [`AnswersV2Query`](./src/models/prompts.rs) |
| Response | [`EventStream<PromptsStreamAnswersV2Response>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .prompts()
    .stream_answers_v2(AnswersV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        include: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

## `Reports`

### Query Citations

Get citations for a given category.

The ``mentioned`` filter supports ``is true`` and ``is false``. It uses the
latest page analysis available at or before ``end_date``; pages without an
analysis by then are excluded from both values. ``citation_share`` keeps all
otherwise eligible citations in its denominator when this filter is used.

| Direction | Type |
| --- | --- |
| Request | [`CitationsQuery`](./src/models/reports.rs) |
| Response | [`CitationsResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .citations(CitationsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![CitationsQueryMetric::Count],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
```

### Query Visibility

Query visibility report.

| Direction | Type |
| --- | --- |
| Request | [`VisibilityQuery`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .visibility(VisibilityQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![VisibilityQueryMetric::ShareOfVoice],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
```

### Query Sentiment

Get citations for a given category.

| Direction | Type |
| --- | --- |
| Request | [`SentimentQuery`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .sentiment(SentimentQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![SentimentQueryMetric::Positive],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
```

### Query Sentiment V2

| Direction | Type |
| --- | --- |
| Request | [`SentimentV2ReportQuery`](./src/models/reports.rs) |
| Response | [`SentimentV2ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .sentiment_v2(SentimentV2ReportQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        asset_name: "".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        comparison_start_date: None,
        comparison_end_date: None,
        date_bucket: None,
        dimensions: None,
        metrics: vec![SentimentV2ReportQueryMetric::Sentiment],
        filters: None,
        order_by: None,
        pagination: None,
    })
    .send()
    .await?;
```

### Get Referrals Report V1

Get referral traffic report from the daily aggregated materialized view.

This endpoint queries pre-aggregated daily referral data, making it efficient
for large date ranges and high-traffic sites.

| Direction | Type |
| --- | --- |
| Request | [`ReferralsQuery`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .get_referrals_report(ReferralsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![ReferralsQueryMetric::Visits],
        order_by: None,
        pagination: None,
        domain: "".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: None,
        organization_id: None,
        metric_filters: None,
        filters: None,
    })
    .send()
    .await?;
```

### Get Bots Report V1

Get bot traffic report from the daily aggregated materialized view.

This endpoint queries pre-aggregated daily bot data, making it efficient
for large date ranges and high-traffic sites.

Metrics:
- count: unique bot visits
- citations: unique citation events
- indexing: unique indexing events
- training: unique training events
- last_visit: most recent visit timestamp

| Direction | Type |
| --- | --- |
| Request | [`BotsReportQuery`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .get_bots_report(BotsReportQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![BotsReportQueryMetric::Count],
        order_by: None,
        pagination: None,
        domain: "".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: None,
        organization_id: None,
        metric_filters: None,
        filters: None,
    })
    .send()
    .await?;
```

### Query Fanouts

| Direction | Type |
| --- | --- |
| Request | [`QueryFanoutsQuery`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .query_fanouts(QueryFanoutsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![QueryFanoutsQueryMetric::FanoutsPerExecution],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
```

### Stream Citations

Stream citations with the same filter semantics as the non-streaming route.

| Direction | Type |
| --- | --- |
| Request | [`StreamCitationsQuery`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamCitationsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_citations(StreamCitationsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![StreamCitationsQueryMetric::Count],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Visibility

| Direction | Type |
| --- | --- |
| Request | [`StreamVisibilityQuery`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamVisibilityResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_visibility(StreamVisibilityQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![StreamVisibilityQueryMetric::ShareOfVoice],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Sentiment

| Direction | Type |
| --- | --- |
| Request | [`StreamSentimentQuery`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamSentimentResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_sentiment(StreamSentimentQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![StreamSentimentQueryMetric::Positive],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Citations V2

| Direction | Type |
| --- | --- |
| Request | [`CitationsV2Query`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamCitationsV2Response>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_citations_v2(CitationsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        entity: None,
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Visibility V2

| Direction | Type |
| --- | --- |
| Request | [`VisibilityV2Query`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamVisibilityV2Response>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_visibility_v2(VisibilityV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        assets: None,
        filter: None,
        sort: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Sentiment V2

| Direction | Type |
| --- | --- |
| Request | [`SentimentV2Query`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamSentimentV2Response>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_sentiment_v2(SentimentV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        asset: "".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        sort: None,
        include_cited_websites: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Stream Query Fanouts V2

| Direction | Type |
| --- | --- |
| Request | [`QueryFanoutsV2Query`](./src/models/reports.rs) |
| Response | [`EventStream<ReportsStreamQueryFanoutsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .stream_query_fanouts(QueryFanoutsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        sort: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### Get Referrals Report V2

Get referral traffic report from the hourly aggregated materialized view (UTC-based).

Supports date_interval="hour", calendar intervals through "year", "quarter", and "relative_week".
When `view_id` is provided, the query is scoped to that domain segment's hosts and paths.

| Direction | Type |
| --- | --- |
| Request | [`ReferralsQueryV2`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .get_referrals_report_v2(ReferralsQueryV2 {
        date_interval: None,
        dimensions: None,
        metrics: vec![ReferralsQueryV2Metric::Visits],
        order_by: None,
        pagination: None,
        domain: "".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: None,
        organization_id: None,
        timezone: None,
        view_id: None,
        metric_filters: None,
        filters: None,
    })
    .send()
    .await?;
```

### Get Bots Report V2

Get bot traffic report from the hourly aggregated materialized view (UTC-based).

Supports date_interval="hour", calendar intervals through "year", "quarter", and "relative_week".
When `view_id` is provided, the query is scoped to that domain segment's hosts and paths.

Metrics:
- count: unique bot visits
- citations: unique citation events (ai_assistant bot type)
- indexing: unique indexing events (index bot type)
- training: unique training events (ai_training bot type)
- last_visit: most recent visit timestamp

Dimensions:
- date, path, bot_name, bot_provider, bot_type

| Direction | Type |
| --- | --- |
| Request | [`BotsReportQueryV2`](./src/models/reports.rs) |
| Response | [`ReportResponse`](./src/models/reports.rs) |

### Query Visibility V2

| Direction | Type |
| --- | --- |
| Request | [`VisibilityV2Query`](./src/models/reports.rs) |
| Response | [`VisibilityV2Response`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .query_visibility(VisibilityV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        assets: None,
        filter: None,
        sort: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

### Query Citations V2

| Direction | Type |
| --- | --- |
| Request | [`CitationsV2Query`](./src/models/reports.rs) |
| Response | [`CitationsV2Response`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .query_citations(CitationsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        entity: None,
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

### Query Sentiment V2

| Direction | Type |
| --- | --- |
| Request | [`SentimentV2Query`](./src/models/reports.rs) |
| Response | [`SentimentV2Response`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .query_sentiment(SentimentV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        asset: "".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        sort: None,
        include_cited_websites: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

### Query Fanouts V2

| Direction | Type |
| --- | --- |
| Request | [`QueryFanoutsV2Query`](./src/models/reports.rs) |
| Response | [`QueryFanoutsV2Response`](./src/models/reports.rs) |

```rust
let response = client
    .reports()
    .query_query_fanouts(QueryFanoutsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        sort: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

### `Reports WebSearchResults`

#### Query Web Search Results

Get web search results for a given category.

| Direction | Type |
| --- | --- |
| Request | [`WebSearchResultsQuery`](./src/models/web_search_results.rs) |
| Response | [`WebSearchResultsResponse`](./src/models/web_search_results.rs) |

```rust
let response = client
    .reports()
    .web_search_results()
    .query(WebSearchResultsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![WebSearchResultsQueryMetric::Count],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
```

#### Stream Web Search Results

| Direction | Type |
| --- | --- |
| Request | [`StreamWebSearchResultsQuery`](./src/models/web_search_results.rs) |
| Response | [`EventStream<WebSearchResultsStreamResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .web_search_results()
    .stream(StreamWebSearchResultsQuery {
        date_interval: None,
        dimensions: None,
        metrics: vec![StreamWebSearchResultsQueryMetric::Count],
        order_by: None,
        pagination: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: chrono::Utc::now().fixed_offset(),
        end_date: chrono::Utc::now().fixed_offset(),
        filters: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### `Reports Shopping`

#### Query Shopping Brands V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingBrandsV2Query`](./src/models/shopping.rs) |
| Response | [`ShoppingBrandsV2Response`](./src/models/shopping.rs) |

```rust
let response = client
    .reports()
    .shopping()
    .brands(ShoppingBrandsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        assets: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

#### Stream Shopping Brands V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingBrandsV2Query`](./src/models/shopping.rs) |
| Response | [`EventStream<ShoppingStreamBrandsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .shopping()
    .stream_brands(ShoppingBrandsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        scope: None,
        assets: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

#### Query Shopping Products V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingProductsV2Query`](./src/models/shopping.rs) |
| Response | [`ShoppingProductsV2Response`](./src/models/shopping.rs) |

```rust
let response = client
    .reports()
    .shopping()
    .products(ShoppingProductsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        include_merchants: None,
        target_product: None,
        competitor_limit: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

#### Stream Shopping Products V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingProductsV2Query`](./src/models/shopping.rs) |
| Response | [`EventStream<ShoppingStreamProductsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .shopping()
    .stream_products(ShoppingProductsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        include_merchants: None,
        target_product: None,
        competitor_limit: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

#### Query Shopping Merchants V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingMerchantsV2Query`](./src/models/shopping.rs) |
| Response | [`ShoppingMerchantsV2Response`](./src/models/shopping.rs) |

```rust
let response = client
    .reports()
    .shopping()
    .merchants(ShoppingMerchantsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

#### Stream Shopping Merchants V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingMerchantsV2Query`](./src/models/shopping.rs) |
| Response | [`EventStream<ShoppingStreamMerchantsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .shopping()
    .stream_merchants(ShoppingMerchantsV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

#### Query Shopping Trigger Rate V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingTriggerRateV2Query`](./src/models/shopping.rs) |
| Response | [`ShoppingTriggerRateV2Response`](./src/models/shopping.rs) |

```rust
let response = client
    .reports()
    .shopping()
    .trigger_rate(ShoppingTriggerRateV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

#### Stream Shopping Trigger Rate V2

| Direction | Type |
| --- | --- |
| Request | [`ShoppingTriggerRateV2Query`](./src/models/shopping.rs) |
| Response | [`EventStream<ShoppingStreamTriggerRateResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .shopping()
    .stream_trigger_rate(ShoppingTriggerRateV2Query {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        metrics: None,
        interval: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### `Reports Accuracy`

#### Accuracy Overview

| Direction | Type |
| --- | --- |
| Request | [`AccuracyOverviewQuery`](./src/models/accuracy.rs) |
| Response | [`AccuracyOverviewResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_overview(AccuracyOverviewQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        date_bucket: None,
        group_by: None,
    })
    .send()
    .await?;
```

#### Accuracy Breakdown

| Direction | Type |
| --- | --- |
| Request | [`AccuracyBreakdownQuery`](./src/models/accuracy.rs) |
| Response | [`AccuracyBreakdownResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_breakdown(AccuracyBreakdownQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        breakdown_by: None,
        group_by: None,
        date_bucket: None,
        limit: None,
        offset: None,
        search_query: None,
        sort_by: None,
        sort_order: None,
        pagination: None,
    })
    .send()
    .await?;
```

#### Accuracy Citation Analysis

| Direction | Type |
| --- | --- |
| Request | [`AccuracyCitationAnalysisQuery`](./src/models/accuracy.rs) |
| Response | [`AccuracyCitationAnalysisResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_citation_analysis(AccuracyCitationAnalysisQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        clean_href: "".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
    })
    .send()
    .await?;
```

#### Accuracy Topic Ids

| Direction | Type |
| --- | --- |
| Request | [`AccuracyTopicIdsQuery`](./src/models/accuracy.rs) |
| Response | `Vec<String>` |

```rust
let response = client
    .reports()
    .accuracy()
    .create_topic_ids(AccuracyTopicIdsQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
    })
    .send()
    .await?;
```

#### Accuracy Inaccurate Themes

| Direction | Type |
| --- | --- |
| Request | [`InaccurateThemesQuery`](./src/models/accuracy.rs) |
| Response | [`InaccurateThemesResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_inaccurate_themes(InaccurateThemesQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        limit: None,
        offset: None,
        sort_by: None,
        sort_order: None,
        search_query: None,
    })
    .send()
    .await?;
```

#### Accuracy Inaccurate Clusters

| Direction | Type |
| --- | --- |
| Request | [`InaccurateClustersQuery`](./src/models/accuracy.rs) |
| Response | [`InaccurateClustersResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_inaccurate_clusters(InaccurateClustersQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        theme_id: None,
        limit: None,
        offset: None,
        search_query: None,
        include_models: None,
    })
    .send()
    .await?;
```

#### Accuracy Inaccuracy Drivers

| Direction | Type |
| --- | --- |
| Request | [`InaccuracyDriversQuery`](./src/models/accuracy.rs) |
| Response | [`InaccuracyDriversResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_inaccuracy_drivers(InaccuracyDriversQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        limit: None,
    })
    .send()
    .await?;
```

#### Accuracy Top Inaccurate Claims

| Direction | Type |
| --- | --- |
| Request | [`TopInaccurateClaimsQuery`](./src/models/accuracy.rs) |
| Response | [`TopInaccurateClaimsResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_top_inaccurate_claims(TopInaccurateClaimsQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        limit: None,
    })
    .send()
    .await?;
```

#### Accuracy Claim Breakdown

| Direction | Type |
| --- | --- |
| Request | [`ClaimBreakdownQuery`](./src/models/accuracy.rs) |
| Response | [`ClaimBreakdownResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_claim_breakdown(ClaimBreakdownQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        cluster_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
    })
    .send()
    .await?;
```

#### Accuracy Claim Citations

| Direction | Type |
| --- | --- |
| Request | [`ClaimCitationsQuery`](./src/models/accuracy.rs) |
| Response | [`ClaimCitationsResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_claim_citations(ClaimCitationsQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        cluster_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        limit: None,
        offset: None,
        search_query: None,
        sort_order: None,
    })
    .send()
    .await?;
```

#### Accuracy Cluster Example Runs

| Direction | Type |
| --- | --- |
| Request | [`ClusterExampleRunsQuery`](./src/models/accuracy.rs) |
| Response | [`ClusterExampleRunsResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_cluster_example_runs(ClusterExampleRunsQuery {
        start_date: "".to_string(),
        end_date: "".to_string(),
        comparison_start_date: None,
        comparison_end_date: None,
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        topic_ids: None,
        exclude_topic_ids: None,
        tag_ids: None,
        tag_filter_type: None,
        include_no_tag: None,
        region_ids: None,
        platform_ids: None,
        persona_ids: None,
        include_no_persona: None,
        prompt_ids: None,
        citation_categories: None,
        cluster_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        limit: None,
        offset: None,
    })
    .send()
    .await?;
```

#### Accuracy Cluster Verification Pairs

| Direction | Type |
| --- | --- |
| Request | [`ClusterVerificationPairsQuery`](./src/models/accuracy.rs) |
| Response | [`ClusterVerificationPairsResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_cluster_verification_pairs(ClusterVerificationPairsQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        cluster_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
    })
    .send()
    .await?;
```

#### Accuracy Factcheck Setup Status

| Direction | Type |
| --- | --- |
| Request | [`FactCheckSetupStatusQuery`](./src/models/accuracy.rs) |
| Response | [`FactCheckSetupStatusResponse`](./src/models/accuracy.rs) |

```rust
let response = client
    .reports()
    .accuracy()
    .create_factcheck_setup_status(FactCheckSetupStatusQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
    })
    .send()
    .await?;
```

### `Reports Factcheck`

#### Query Scores

| Direction | Type |
| --- | --- |
| Request | [`FactcheckScoresQuery`](./src/models/factcheck.rs) |
| Response | [`FactcheckScoresResponse`](./src/models/factcheck.rs) |

```rust
let response = client
    .reports()
    .factcheck()
    .query_scores(FactcheckScoresQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

#### Stream Scores

| Direction | Type |
| --- | --- |
| Request | [`FactcheckScoresQuery`](./src/models/factcheck.rs) |
| Response | [`EventStream<FactcheckStreamScoresResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .factcheck()
    .stream_scores(FactcheckScoresQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        filter: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

#### `Reports Factcheck Claims`

##### Query Claims

| Direction | Type |
| --- | --- |
| Request | [`FactcheckClaimsQuery`](./src/models/claims.rs) |
| Response | [`FactcheckClaimsResponse`](./src/models/claims.rs) |

```rust
let response = client
    .reports()
    .factcheck()
    .claims()
    .query_claims(FactcheckClaimsQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        filter: None,
        include: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
```

##### Stream Claims

| Direction | Type |
| --- | --- |
| Request | [`FactcheckClaimsQuery`](./src/models/claims.rs) |
| Response | [`EventStream<ClaimsStreamClaimsResponse>`](./src/streaming.rs) |

```rust
use futures::StreamExt;
let mut events = client
    .reports()
    .factcheck()
    .claims()
    .stream_claims(FactcheckClaimsQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        group_by: None,
        filter: None,
        include: None,
        limit: None,
        max_results: None,
        cursor: None,
    })
    .send()
    .await?;
if let Some(event) = events.next().await {
    event?;
}
```

### `Reports Social`

#### `Reports Social Youtube`

##### Query Youtube Channels

Rank the YouTube channels cited in a category, or the video categories they publish in.

| Direction | Type |
| --- | --- |
| Request | [`YoutubeChannelsQuery`](./src/models/youtube.rs) |
| Response | [`YoutubeChannelsResponse`](./src/models/youtube.rs) |

```rust
let response = client
    .reports()
    .social()
    .youtube()
    .get_channels(YoutubeChannelsQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        filter: None,
        limit: None,
        cursor: None,
        source_types: None,
        group_by: None,
        interval: None,
    })
    .send()
    .await?;
```

##### Query Youtube Videos

Rank cited YouTube videos, for one channel or across all of them.

| Direction | Type |
| --- | --- |
| Request | [`YoutubeVideosQuery`](./src/models/youtube.rs) |
| Response | [`YoutubeVideosResponse`](./src/models/youtube.rs) |

```rust
let response = client
    .reports()
    .social()
    .youtube()
    .get_videos(YoutubeVideosQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        filter: None,
        limit: None,
        cursor: None,
        source_types: None,
        attribution: None,
    })
    .send()
    .await?;
```

##### Query Youtube Summary

Report how much of youtube.com the channel and video rankings account for.

| Direction | Type |
| --- | --- |
| Request | [`YoutubeSummaryQuery`](./src/models/youtube.rs) |
| Response | [`YoutubeSummaryResponse`](./src/models/youtube.rs) |

```rust
let response = client
    .reports()
    .social()
    .youtube()
    .get_summary(YoutubeSummaryQuery {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        start_date: "".to_string(),
        end_date: "".to_string(),
        filter: None,
    })
    .send()
    .await?;
```

## `Content`

### `Content Optimization`

#### Optimization List

| Direction | Type |
| --- | --- |
| Response | [`ContentOptimizationResponse`](./src/models/optimization.rs) |

```rust
let response = client
    .content()
    .optimization()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Optimization Analysis

| Direction | Type |
| --- | --- |
| Response | [`ContentOptimizationAnalysisResponse`](./src/models/optimization.rs) |

```rust
let response = client
    .content()
    .optimization()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

## `Agents`

### List agents

List agents available to your organization.

Agent status reflects whether an agent has ever been published. `published`
agents have a live published version. `draft` agents have not been
published yet.

| Direction | Type |
| --- | --- |
| Response | [`ListAgentsResponse`](./src/models/agents.rs) |

```rust
let response = client.agents().list().send().await?;
```

### Get an agent

Retrieve an agent and its schema details.

Agents can have both a live published version and a draft version with newer
unpublished changes. Use the `version` parameter to choose which state to return.

| Direction | Type |
| --- | --- |
| Response | [`AgentDetail`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Create an agent

Create a new draft agent owned by the given organization.

`organization_id` is required and you must be a member of it. The agent is created
as a `draft`; publish it with `POST /v1/agents/{agent_id}/publish` once its graph
is ready.

| Direction | Type |
| --- | --- |
| Request | [`CreateAgentRequest`](./src/models/agents.rs) |
| Response | [`Agent`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .create(CreateAgentRequest {
        organization_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        name: "x".to_string(),
        description: None,
        graph: None,
    })
    .send()
    .await?;
```

### Publish an agent

Publish an agent's latest draft as its live published version.

You must be a member of the agent's organization. Publishing promotes the current
draft graph to a new published version. A draft that cannot produce its declared
input/output contract is rejected with `422` and is not published.

| Direction | Type |
| --- | --- |
| Response | [`Agent`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .publish("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update an agent

Update an agent's draft graph in place.

You must be a member of the agent's organization. The agent's draft is replaced with the
supplied graph and re-validated, so you can iterate one draft — create, then update per
fix — instead of creating a new agent on every change. The response carries the updated
`validation`; publish with `POST /v1/agents/{agent_id}/publish` once `validation.valid`.

| Direction | Type |
| --- | --- |
| Request | [`UpdateAgentRequest`](./src/models/agents.rs) |
| Response | [`AgentDetail`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdateAgentRequest {
            graph: serde_json::json!(null),
        },
    )
    .send()
    .await?;
```

### Get an agent's graph

Retrieve an agent's full workflow graph (`{nodes, edges}`).

The graph is returned verbatim in the canonical dialect — the same shape `POST /v1/agents`
and `PATCH /v1/agents/{agent_id}` accept — so a known-good agent can be read back, copied,
and edited. Tool-backed nodes appear in their lowered `tool` form rather than the friendly
v1 node types. A `draft` is visible only to its creator; the `published` version is visible
across its organization.

| Direction | Type |
| --- | --- |
| Response | [`AgentGraph`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .retrieve_graph("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Agents Runs`

#### Run an agent

Start a new run for an agent.

Runs always execute the agent's live published version, so the agent must be
published first with `POST /v1/agents/{agent_id}/publish`. Unpublished drafts
cannot be run.

| Direction | Type |
| --- | --- |
| Request | [`RunAgentRequest`](./src/models/runs.rs) |
| Response | [`AcceptedAgentRun`](./src/models/runs.rs) |

```rust
let response = client
    .agents()
    .runs()
    .create("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get an agent run

Retrieve the current status and result details for an agent run.

| Direction | Type |
| --- | --- |
| Response | [`AgentRun`](./src/models/runs.rs) |

```rust
let response = client
    .agents()
    .runs()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Agents NodeTypes`

#### List node types

List the node types available for building agents.

The set is deterministic and does not depend on the caller, so the response
is safe to cache across sessions. Integration-dependent and dynamic-schema
node types are intentionally excluded in v1.

| Direction | Type |
| --- | --- |
| Response | [`ListNodeTypesResponse`](./src/models/node_types.rs) |

```rust
let response = client.agents().node_types().list().send().await?;
```

#### Get a node type schema

Retrieve the JSON schema for a single node type.

The `schema` field is an opaque JSON Schema for the node's configuration.
Use `schema_version` as a cache key — it bumps whenever the schema changes.

| Direction | Type |
| --- | --- |
| Response | [`NodeSchemaResponse`](./src/models/node_types.rs) |

```rust
let response = client.agents().node_types().retrieve_schema("example").send().await?;
```

## `KnowledgeBases`

### List Knowledge Bases

List knowledge bases accessible to the API key.

| Direction | Type |
| --- | --- |
| Response | [`ListKnowledgeBasesResponse`](./src/models/knowledge_bases.rs) |

```rust
let response = client.knowledge_bases().list().send().await?;
```

### Search Knowledge Base

Search a knowledge base and return matching snippets or pages.

| Direction | Type |
| --- | --- |
| Request | [`SearchKnowledgeBaseRequest`](./src/models/knowledge_bases.rs) |
| Response | [`KnowledgeBaseSearchResponse`](./src/models/knowledge_bases.rs) |

```rust
let response = client
    .knowledge_bases()
    .search(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SearchKnowledgeBaseRequest {
            query: "x".to_string(),
            top_k: 0,
            return_full_page: None,
            filters: None,
        },
    )
    .send()
    .await?;
```

### `KnowledgeBases Documents`

#### Add Document

Add a document to a knowledge base using JSON text or multipart file upload.

| Direction | Type |
| --- | --- |
| Request | [`DocumentsCreateBody`](./src/models/knowledge_bases_documents.rs) |
| Response | [`DocumentOperationResponse`](./src/models/knowledge_bases_documents.rs) |

```rust
let response = client
    .knowledge_bases()
    .documents()
    .create(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        DocumentsCreateBody {
            name: "x".to_string(),
            text: "x".to_string(),
            folder: None,
        },
    )
    .send()
    .await?;
```

#### Update Document

Overwrite a knowledge base document using JSON text or multipart file upload.

| Direction | Type |
| --- | --- |
| Request | [`DocumentsUpdateBody`](./src/models/knowledge_bases_documents.rs) |
| Response | [`DocumentOperationResponse`](./src/models/knowledge_bases_documents.rs) |

```rust
let response = client
    .knowledge_bases()
    .documents()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        DocumentsUpdateBody {
            name: "x".to_string(),
            text: "x".to_string(),
            folder: None,
        },
    )
    .send()
    .await?;
```

#### Delete Document

Delete an existing document from a knowledge base.

| Direction | Type |
| --- | --- |
| Request | [`DeleteDocumentRequest`](./src/models/knowledge_bases_documents.rs) |
| Response | [`DocumentOperationResponse`](./src/models/knowledge_bases_documents.rs) |

```rust
let response = client
    .knowledge_bases()
    .documents()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        DeleteDocumentRequest { name: "x".to_string() },
    )
    .send()
    .await?;
```

### `KnowledgeBases Folders`

#### Add Folder

Create an empty folder at the requested knowledge base path.

| Direction | Type |
| --- | --- |
| Request | [`AddFolderRequest`](./src/models/folders.rs) |
| Response | [`FolderOperationResponse`](./src/models/folders.rs) |

```rust
let response = client
    .knowledge_bases()
    .folders()
    .create(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        AddFolderRequest { path: "x".to_string() },
    )
    .send()
    .await?;
```

#### Delete Folder

Delete a folder. With recursive=false, non-empty folders return 409 and no contents are deleted.

| Direction | Type |
| --- | --- |
| Request | [`DeleteFolderRequest`](./src/models/folders.rs) |
| Response | [`FolderOperationResponse`](./src/models/folders.rs) |

```rust
let response = client
    .knowledge_bases()
    .folders()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        DeleteFolderRequest {
            path: "x".to_string(),
            recursive: None,
        },
    )
    .send()
    .await?;
```

## `Projects`

### List Projects

| Direction | Type |
| --- | --- |
| Response | [`ListProjectsResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Create Project

| Direction | Type |
| --- | --- |
| Request | [`CreateProjectRequest`](./src/models/projects.rs) |
| Response | [`ProjectGenerationResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .create(CreateProjectRequest {
        category_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        title: None,
        project_name: None,
        focus: None,
        topics: None,
        attachments: None,
        generation_context: None,
    })
    .send()
    .await?;
```

### Get Project

| Direction | Type |
| --- | --- |
| Response | [`ProjectDetailResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Delete Project

| Direction | Type |
| --- | --- |
| Response | `()` |

```rust
client
    .projects()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Get Project Status

| Direction | Type |
| --- | --- |
| Response | [`ProjectStatusResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .get_status(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Archive Project

| Direction | Type |
| --- | --- |
| Request | [`ArchiveProjectRequest`](./src/models/projects.rs) |
| Response | [`ProjectResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .archive(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Unarchive Project

| Direction | Type |
| --- | --- |
| Response | [`ProjectResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .unarchive(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Projects Generations`

#### List Project Generations

| Direction | Type |
| --- | --- |
| Response | [`ListProjectGenerationsResponse`](./src/models/generations.rs) |

```rust
let response = client
    .projects()
    .generations()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Get Project Generation Status

| Direction | Type |
| --- | --- |
| Response | [`ProjectGenerationStatusResponse`](./src/models/generations.rs) |

```rust
let response = client
    .projects()
    .generations()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Projects Tasks`

#### List Project Tasks

| Direction | Type |
| --- | --- |
| Response | [`ListProjectTasksResponse`](./src/models/tasks.rs) |

```rust
let response = client
    .projects()
    .tasks()
    .list(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

#### Create Project Task

| Direction | Type |
| --- | --- |
| Request | [`CreateProjectTaskRequest`](./src/models/tasks.rs) |
| Response | [`ProjectTaskResponse`](./src/models/tasks.rs) |

```rust
let response = client
    .projects()
    .tasks()
    .create(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        CreateProjectTaskRequest {
            title: "x".to_string(),
            summary: None,
            brief: None,
            r#type: None,
            topic: None,
            impact: None,
            reference_url: None,
            reference_label: None,
            position: None,
        },
    )
    .send()
    .await?;
```

#### Get Project Task

| Direction | Type |
| --- | --- |
| Response | [`ProjectTaskDetailResponse`](./src/models/tasks.rs) |

```rust
let response = client
    .projects()
    .tasks()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

#### Update Project Task

| Direction | Type |
| --- | --- |
| Request | [`UpdateProjectTaskRequest`](./src/models/tasks.rs) |
| Response | [`ProjectTaskResponse`](./src/models/tasks.rs) |

```rust
let response = client
    .projects()
    .tasks()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdateProjectTaskRequest {
            title: None,
            summary: None,
            brief: None,
            r#type: None,
            topic: None,
            impact: None,
            reference_url: None,
            reference_label: None,
        },
    )
    .send()
    .await?;
```

#### Delete Project Task

| Direction | Type |
| --- | --- |
| Response | `()` |

```rust
client
    .projects()
    .tasks()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

#### Update Project Task Status

| Direction | Type |
| --- | --- |
| Request | [`UpdateProjectTaskStatusRequest`](./src/models/tasks.rs) |
| Response | [`ProjectTaskStatusResponse`](./src/models/tasks.rs) |

```rust
let response = client
    .projects()
    .tasks()
    .update_status(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdateProjectTaskStatusRequest {
            status: UpdateProjectTaskStatusRequestStatus::NotStarted,
            note: None,
        },
    )
    .send()
    .await?;
```

## `Integrations`

### List Integrations

List the organization's connected integrations.

Returns every connected integration by default, each with its lifecycle
`status`; pass `status_filter` to narrow to one status (e.g. `needs_reauth`).
Each row's `integration_id` is the value a hub-backed node needs bound to it.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsResponse`](./src/models/integrations.rs) |

```rust
let response = client.integrations().list().send().await?;
```

## `Documents`

### Create a document

Create a Profound document with markdown content.

`organization_id` is required and you must be a member of it. You choose the
document's `id`, and creation is idempotent on it: repeating the request returns
the existing document rather than creating a second one.

New documents are visible only to their creator; share them from the Profound app,
or open one with the `url` in the response.

A `201` response does not confirm that a new document was created: it is also
returned when `id` already existed, in which case the existing document comes
back unchanged. Upstream gives no signal to tell the two apart, so this endpoint
does not claim to either — it is safe to retry with the same `id` either way.

| Direction | Type |
| --- | --- |
| Request | [`CreateDocumentRequest`](./src/models/documents.rs) |
| Response | [`CreateDocumentResponse`](./src/models/documents.rs) |

```rust
let response = client
    .documents()
    .create(CreateDocumentRequest {
        id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        organization_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        name: "x".to_string(),
        content_markdown: "x".to_string(),
    })
    .send()
    .await?;
```

### List documents

List documents visible to your organization, newest-modified-first.

Documents are ordered by last-modified time, most recent first, with no other
sort option. This is a walk over a live, mutable collection: a document created
or modified while you are paging can shift which page it lands on, so a single
walk may show it to you twice or, rarely, skip it.

This response never includes a total count. Upstream counts totals before
applying your organization's access filter, so a total, or treating a short
page as the last one, would misreport what you can actually see. Keep
following `pagination.next_cursor` until it comes back null — that, and not
a short or even an empty page, is the end of the walk. A page whose rows the
access filter removed entirely is empty while later pages still hold
documents, so the last page of a walk may legitimately be an empty one.

| Direction | Type |
| --- | --- |
| Response | [`ListDocumentsResponse`](./src/models/documents.rs) |

```rust
let response = client
    .documents()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Read a document

Read a document: its metadata, its default tab's body, its other tabs, its comments, and its version hash.

You can read any document you have access to in the Profound app, including ones
created there rather than through this API.

By default this is a preview: the body is truncated to save your context, and the
version hash is withheld so a preview alone can never be used to replace a document
blindly. Pass `preview=false` when you intend to write.

| Direction | Type |
| --- | --- |
| Response | [`ReadDocumentResponse`](./src/models/documents.rs) |

```rust
let response = client
    .documents()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Rename or reshare a document

Rename a document, change who can see it, or both in one call.

Renaming sets a permanent lock on the title, and changing visibility can silently
change who has access — see the `name` and `visibility` field descriptions for what
each one does before you use it.

Renaming needs edit access; changing visibility is creator-only, and upstream
enforces it. You can act on a document this API created, or one you created
yourself in the Profound app — not one merely shared with you.

| Direction | Type |
| --- | --- |
| Request | [`UpdateDocumentRequest`](./src/models/documents.rs) |
| Response | [`Document`](./src/models/documents.rs) |

```rust
let response = client
    .documents()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        UpdateDocumentRequest {
            organization_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
            name: None,
            visibility: None,
        },
    )
    .send()
    .await?;
```

### Delete a document

Delete a document created through this integration.

Only documents created through this integration can be deleted here. A document
created in the Profound app can never be deleted through this route, even by the
person who owns it — creation provenance is stamped once, at creation, and is never
backfilled onto documents made another way.

The delete is soft: the row is marked deleted at the storage layer rather than
destroyed. There is no restore through this API, or any other — treat a delete as
final even though the data itself is not gone.

A 404 means the document is not visible to you at all. It covers three cases the
response does not distinguish, on purpose: the document never existed, it was
already deleted by an earlier call to this same route, or it exists but your
credential resolves no role on it. Deleting the same document twice returns 404 on
the second call, not a second 204.

A 403 means the opposite: the document is visible to you but not deletable here,
and the message says which rule refused — it was not created through this
integration, or you are not its creator. Deleting is creator-only, so edit access
is not enough to remove a document out from under its owner.

| Direction | Type |
| --- | --- |
| Response | `()` |

```rust
client
    .documents()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### Replace a document's content

Overwrite a document's entire body with new markdown, replacing what it held before.

This is a whole-body replace, not a patch: send the complete new text every time. An
empty `content_markdown` is valid and clears the document.

Two destructive side effects apply on every call, regardless of what you send:

- The document collapses to its default tab. Every non-default tab is deleted, and
  the comments map is cleared for **all** tabs, including the default one — a
  document with a live comment thread on any tab loses it.
- `skip_title_sync` defaults to `false`, matching the Profound app: the title follows
  the new content's first heading, so a replace silently renames the document unless
  the heading matches the current title or `skip_title_sync` is set.

There is no compare-and-swap: this call does not accept a precondition, and nothing
stops two concurrent replaces from silently overwriting each other last-writer-wins.
Upstream's own `version_hash` documentation says as much — the token is "still a
change detector rather than a precondition: a caller must not treat a matching token
as licence to overwrite blindly, because it names the room at a moment cortex
observed and not the moment its own write lands." Sending a `working_version_hash`
(or any spelling of it) is rejected with a `400` naming this rather than accepted
and silently discarded, which is what happens on the upstream route this wraps.

You can replace a document this API created, or one you created yourself directly —
not merely one shared with you.

| Direction | Type |
| --- | --- |
| Request | [`ReplaceDocumentContentRequest`](./src/models/documents.rs) |
| Response | [`ReplaceDocumentContentResponse`](./src/models/documents.rs) |

```rust
let response = client
    .documents()
    .replace_content(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        ReplaceDocumentContentRequest {
            organization_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
            content_markdown: "".to_string(),
            skip_title_sync: None,
        },
    )
    .send()
    .await?;
```

## `Ads`

### `Ads OpenaiAds`

#### `Ads OpenaiAds AdAccount`

##### Get Account Insights

Get ad account insights for the organization's OpenAI Ads partner brand.

`aggregation_level=campaign` returns one row per campaign (with `campaign_id`
/ `campaign_name` and all metrics), so every campaign's insights come back in
a single call; `time_granularity=daily` gives per-day rows (e.g. daily spend).

| Direction | Type |
| --- | --- |
| Response | [`InsightsResponse`](./src/models/ad_account.rs) |

```rust
let response = client
    .ads()
    .openai_ads()
    .ad_account()
    .retrieve_insights()
    .send()
    .await?;
```
