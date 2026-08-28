# Profound Rust SDK Reference

## Operations

### client.organizations().regions()

Get Regions

- HTTP: `GET /v1/org/regions`
- Response body: `application/json`
- Errors: `422`

### client.organizations().models()

Get Models

- HTTP: `GET /v1/org/models`
- Response body: `application/json`
- Errors: `422`

### client.organizations().domains()

Get Domains

- HTTP: `GET /v1/org/domains`
- Response body: `application/json`
- Errors: `422`

### client.organizations().list_assets()

Get Assets

- HTTP: `GET /v1/org/assets`
- Response body: `application/json`
- Errors: `422`

### client.organizations().get_personas()

Get Personas

- HTTP: `GET /v1/org/personas`
- Response body: `application/json`
- Errors: `422`

### client.organizations().list()

List organizations

- HTTP: `GET /v1/org`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().list()

Get Categories

- HTTP: `GET /v1/org/categories`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().topics(…)

Get Category Topics

- HTTP: `GET /v1/org/categories/{category_id}/topics`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().tags(…)

Get Category Tags

- HTTP: `GET /v1/org/categories/{category_id}/tags`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().prompts(…)

List prompts

- HTTP: `GET /v1/org/categories/{category_id}/prompts`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().assets(…)

Get Category Assets

- HTTP: `GET /v1/org/categories/{category_id}/assets`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().get_category_personas(…)

Get Category Personas

- HTTP: `GET /v1/org/categories/{category_id}/personas`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().create_prompts(…)

Create prompts

- HTTP: `POST /v1/org/categories/{category_id}/prompts`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().update_prompts(…)

Update prompts

- HTTP: `PATCH /v1/org/categories/{category_id}/prompts`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().update_prompt_status(…)

Update prompt status

- HTTP: `PATCH /v1/org/categories/{category_id}/prompts/status`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().retrieve_regions(…)

Get Category Regions

- HTTP: `GET /v1/org/categories/{category_id}/regions`
- Response body: `application/json`
- Errors: `422`

### client.organizations().categories().get_citation_categories(…)

Get Category Citation Categories

- HTTP: `GET /v1/org/categories/{category_id}/citation-categories`
- Response body: `application/json`
- Errors: `422`

### client.prompts().answers(…)

Get Answers

- HTTP: `POST /v1/prompts/answers`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.prompts().answers_v2(…)

Query Answers V2

- HTTP: `POST /v2/prompts/answers`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.prompts().stream_answers_v2(…)

Stream Answers V2

- HTTP: `POST /v2/prompts/answers/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().citations(…)

Query Citations

- HTTP: `POST /v1/reports/citations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().visibility(…)

Query Visibility

- HTTP: `POST /v1/reports/visibility`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().sentiment(…)

Query Sentiment

- HTTP: `POST /v1/reports/sentiment`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().sentiment_v2(…)

Query Sentiment V2

- HTTP: `POST /v1/reports/sentiment-v2`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().get_referrals_report(…)

Get Referrals Report V1

- HTTP: `POST /v1/reports/referrals`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().get_bots_report(…)

Get Bots Report V1

- HTTP: `POST /v1/reports/bots`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().query_fanouts(…)

Query Fanouts

- HTTP: `POST /v1/reports/query-fanouts`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().stream_citations(…)

Stream Citations

- HTTP: `POST /v1/reports/citations/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_visibility(…)

Stream Visibility

- HTTP: `POST /v1/reports/visibility/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_sentiment(…)

Stream Sentiment

- HTTP: `POST /v1/reports/sentiment/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_citations_v2(…)

Stream Citations V2

- HTTP: `POST /v2/reports/citations/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_visibility_v2(…)

Stream Visibility V2

- HTTP: `POST /v2/reports/visibility/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_sentiment_v2(…)

Stream Sentiment V2

- HTTP: `POST /v2/reports/sentiment/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().stream_query_fanouts(…)

Stream Query Fanouts V2

- HTTP: `POST /v2/reports/query-fanouts/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().get_referrals_report_v2(…)

Get Referrals Report V2

- HTTP: `POST /v2/reports/referrals`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().get_bots_report_v2(…)

Get Bots Report V2

- HTTP: `POST /v2/reports/bots`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().query_visibility(…)

Query Visibility V2

- HTTP: `POST /v2/reports/visibility`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().query_citations(…)

Query Citations V2

- HTTP: `POST /v2/reports/citations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().query_sentiment(…)

Query Sentiment V2

- HTTP: `POST /v2/reports/sentiment`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().query_query_fanouts(…)

Query Fanouts V2

- HTTP: `POST /v2/reports/query-fanouts`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().web_search_results().query(…)

Query Web Search Results

- HTTP: `POST /v1/reports/web-search-results`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().web_search_results().stream(…)

Stream Web Search Results

- HTTP: `POST /v1/reports/web-search-results/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().shopping().brands(…)

Query Shopping Brands V2

- HTTP: `POST /v2/reports/shopping/brands`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().shopping().stream_brands(…)

Stream Shopping Brands V2

- HTTP: `POST /v2/reports/shopping/brands/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().shopping().products(…)

Query Shopping Products V2

- HTTP: `POST /v2/reports/shopping/products`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().shopping().stream_products(…)

Stream Shopping Products V2

- HTTP: `POST /v2/reports/shopping/products/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().shopping().merchants(…)

Query Shopping Merchants V2

- HTTP: `POST /v2/reports/shopping/merchants`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().shopping().stream_merchants(…)

Stream Shopping Merchants V2

- HTTP: `POST /v2/reports/shopping/merchants/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().shopping().trigger_rate(…)

Query Shopping Trigger Rate V2

- HTTP: `POST /v2/reports/shopping/trigger-rate`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().shopping().stream_trigger_rate(…)

Stream Shopping Trigger Rate V2

- HTTP: `POST /v2/reports/shopping/trigger-rate/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().accuracy().create_overview(…)

Accuracy Overview

- HTTP: `POST /v1/reports/accuracy/overview`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_breakdown(…)

Accuracy Breakdown

- HTTP: `POST /v1/reports/accuracy/breakdown`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_citation_analysis(…)

Accuracy Citation Analysis

- HTTP: `POST /v1/reports/accuracy/citation-analysis`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_topic_ids(…)

Accuracy Topic Ids

- HTTP: `POST /v1/reports/accuracy/topic-ids`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_inaccurate_themes(…)

Accuracy Inaccurate Themes

- HTTP: `POST /v1/reports/accuracy/inaccurate-themes`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_inaccurate_clusters(…)

Accuracy Inaccurate Clusters

- HTTP: `POST /v1/reports/accuracy/inaccurate-clusters`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_inaccuracy_drivers(…)

Accuracy Inaccuracy Drivers

- HTTP: `POST /v1/reports/accuracy/inaccuracy-drivers`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_top_inaccurate_claims(…)

Accuracy Top Inaccurate Claims

- HTTP: `POST /v1/reports/accuracy/top-inaccurate-claims`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_claim_breakdown(…)

Accuracy Claim Breakdown

- HTTP: `POST /v1/reports/accuracy/claim-breakdown`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_claim_citations(…)

Accuracy Claim Citations

- HTTP: `POST /v1/reports/accuracy/claim-citations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_cluster_example_runs(…)

Accuracy Cluster Example Runs

- HTTP: `POST /v1/reports/accuracy/cluster-example-runs`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_cluster_verification_pairs(…)

Accuracy Cluster Verification Pairs

- HTTP: `POST /v1/reports/accuracy/cluster-verification-pairs`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().accuracy().create_factcheck_setup_status(…)

Accuracy Factcheck Setup Status

- HTTP: `POST /v1/reports/accuracy/factcheck-setup-status`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().factcheck().query_scores(…)

Query Scores

- HTTP: `POST /v2/reports/factcheck`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().factcheck().stream_scores(…)

Stream Scores

- HTTP: `POST /v2/reports/factcheck/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().factcheck().claims().query_claims(…)

Query Claims

- HTTP: `POST /v2/reports/factcheck/claims`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().factcheck().claims().stream_claims(…)

Stream Claims

- HTTP: `POST /v2/reports/factcheck/claims/stream`
- Request body: `application/json`
- Response body: `text/event-stream`
- Streaming: `sse`
- Errors: `422`

### client.reports().social().youtube().get_channels(…)

Query Youtube Channels

- HTTP: `POST /v2/reports/social/youtube/channels`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().social().youtube().get_videos(…)

Query Youtube Videos

- HTTP: `POST /v2/reports/social/youtube/videos`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.reports().social().youtube().get_summary(…)

Query Youtube Summary

- HTTP: `POST /v2/reports/social/youtube/summary`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.content().optimization().list(…)

Optimization List

- HTTP: `GET /v1/content/{asset_id}/optimization`
- Response body: `application/json`
- Errors: `422`

### client.content().optimization().retrieve(…)

Optimization Analysis

- HTTP: `GET /v1/content/{asset_id}/optimization/{content_id}`
- Response body: `application/json`
- Errors: `422`

### client.agents().list()

List agents

- HTTP: `GET /v1/agents`
- Response body: `application/json`
- Errors: `422`

### client.agents().retrieve(…)

Get an agent

- HTTP: `GET /v1/agents/{agent_id}`
- Response body: `application/json`
- Errors: `422`

### client.agents().create(…)

Create an agent

- HTTP: `POST /v1/agents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.agents().publish(…)

Publish an agent

- HTTP: `POST /v1/agents/{agent_id}/publish`
- Response body: `application/json`
- Errors: `422`

### client.agents().update(…)

Update an agent

- HTTP: `PATCH /v1/agents/{agent_id}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.agents().retrieve_graph(…)

Get an agent's graph

- HTTP: `GET /v1/agents/{agent_id}/graph`
- Response body: `application/json`
- Errors: `422`

### client.agents().runs().create(…)

Run an agent

- HTTP: `POST /v1/agents/{agent_id}/runs`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.agents().runs().retrieve(…)

Get an agent run

- HTTP: `GET /v1/agents/{agent_id}/runs/{run_id}`
- Response body: `application/json`
- Errors: `422`

### client.agents().node_types().list()

List node types

- HTTP: `GET /v1/agents/node-types`
- Response body: `application/json`
- Errors: `422`

### client.agents().node_types().retrieve_schema(…)

Get a node type schema

- HTTP: `GET /v1/agents/node-types/{node_type}/schema`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().list()

List Knowledge Bases

- HTTP: `GET /v1/knowledge-bases`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().search(…)

Search Knowledge Base

- HTTP: `POST /v1/knowledge-bases/{knowledge_base_id}/search`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().documents().create(…)

Add Document

- HTTP: `POST /v1/knowledge-bases/{knowledge_base_id}/documents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().documents().update(…)

Update Document

- HTTP: `PUT /v1/knowledge-bases/{knowledge_base_id}/documents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().documents().delete(…)

Delete Document

- HTTP: `DELETE /v1/knowledge-bases/{knowledge_base_id}/documents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().folders().create(…)

Add Folder

- HTTP: `POST /v1/knowledge-bases/{knowledge_base_id}/folders`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.knowledge_bases().folders().delete(…)

Delete Folder

- HTTP: `DELETE /v1/knowledge-bases/{knowledge_base_id}/folders`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.projects().list(…)

List Projects

- HTTP: `GET /v1/projects`
- Response body: `application/json`
- Errors: `422`

### client.projects().create(…)

Create Project

- HTTP: `POST /v1/projects`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.projects().retrieve(…)

Get Project

- HTTP: `GET /v1/projects/{project_id}`
- Response body: `application/json`
- Errors: `422`

### client.projects().delete(…)

Delete Project

- HTTP: `DELETE /v1/projects/{project_id}`
- Errors: `422`

### client.projects().get_status(…)

Get Project Status

- HTTP: `GET /v1/projects/{project_id}/status`
- Response body: `application/json`
- Errors: `422`

### client.projects().archive(…)

Archive Project

- HTTP: `POST /v1/projects/{project_id}/archive`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.projects().unarchive(…)

Unarchive Project

- HTTP: `POST /v1/projects/{project_id}/unarchive`
- Response body: `application/json`
- Errors: `422`

### client.projects().generations().list(…)

List Project Generations

- HTTP: `GET /v1/projects/generations`
- Response body: `application/json`
- Errors: `422`

### client.projects().generations().retrieve(…)

Get Project Generation Status

- HTTP: `GET /v1/projects/generations/{run_id}`
- Response body: `application/json`
- Errors: `422`

### client.projects().tasks().list(…)

List Project Tasks

- HTTP: `GET /v1/projects/{project_id}/tasks`
- Response body: `application/json`
- Errors: `422`

### client.projects().tasks().create(…)

Create Project Task

- HTTP: `POST /v1/projects/{project_id}/tasks`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.projects().tasks().retrieve(…)

Get Project Task

- HTTP: `GET /v1/projects/{project_id}/tasks/{task_id}`
- Response body: `application/json`
- Errors: `422`

### client.projects().tasks().update(…)

Update Project Task

- HTTP: `PATCH /v1/projects/{project_id}/tasks/{task_id}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.projects().tasks().delete(…)

Delete Project Task

- HTTP: `DELETE /v1/projects/{project_id}/tasks/{task_id}`
- Errors: `422`

### client.projects().tasks().update_status(…)

Update Project Task Status

- HTTP: `POST /v1/projects/{project_id}/tasks/{task_id}/status`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.integrations().list()

List Integrations

- HTTP: `GET /v1/integrations`
- Response body: `application/json`
- Errors: `422`

### client.documents().create(…)

Create a document

- HTTP: `POST /v1/documents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.documents().list(…)

List documents

- HTTP: `GET /v1/documents`
- Response body: `application/json`
- Errors: `422`

### client.documents().retrieve(…)

Read a document

- HTTP: `GET /v1/documents/{document_id}`
- Response body: `application/json`
- Errors: `422`

### client.documents().update(…)

Rename or reshare a document

- HTTP: `PATCH /v1/documents/{document_id}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.documents().delete(…)

Delete a document

- HTTP: `DELETE /v1/documents/{document_id}`
- Errors: `422`

### client.documents().replace_content(…)

Replace a document's content

- HTTP: `POST /v1/documents/{document_id}/content`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `422`

### client.ads().openai_ads().ad_account().retrieve_insights()

Get Account Insights

- HTTP: `GET /v1/ads/openai-ads/ad-account/insights`
- Response body: `application/json`
- Errors: `422`

## Models

- `AeoScore`
- `AeoScoreTargetZone`
- `AcceptedAgentRun`
- `AccuracyBreakdownQuery`
- `AccuracyBreakdownResponse`
- `AccuracyBreakdownRow`
- `AccuracyCitationAnalysisQuery`
- `AccuracyCitationAnalysisResponse`
- `AccuracyCitationClaim`
- `AccuracyCitationEvidence`
- `AccuracyOverviewQuery`
- `AccuracyOverviewResponse`
- `AccuracyPagination`
- `AccuracyScoreBreakdown`
- `AccuracyThemeTrendPoint`
- `AccuracyThemeTrendSeries`
- `AccuracyTopicIdsQuery`
- `AccuracyTrendPoint`
- `AccuracyTrendSeriesMeta`
- `AddFolderRequest`
- `AdditionalTab`
- `Agent`
- `AgentDetail`
- `AgentGraph`
- `AgentRun`
- `AgentRunStatus`
- `AgentRunStep`
- `AgentSchema`
- `AgentStatus`
- `AgentValidation`
- `AgentValidationIssue`
- `AgentVersion`
- `AnalysisBreakdown`
- `AnalysisInputs`
- `AnalysisSummary`
- `AnalysisType`
- `AnalysisTypeFilter`
- `AnswerRow`
- `AnswersQuery`
- `AnswersRawData`
- `AnswersResponse`
- `AnswersResponseInfo`
- `AnswersV2Info`
- `AnswersV2Query`
- `AnswersV2Response`
- `ArchiveProjectRequest`
- `AssetIdFilter`
- `AssetRef`
- `BotNameFilter`
- `BotProviderFilter`
- `BotTypeFilter`
- `BotsReportQuery`
- `BotsReportQueryV2`
- `BrandNameFilter`
- `Category`
- `CategoryAsset`
- `CategoryAssetWithCategory`
- `CategoryPersona`
- `CategoryPersonasResponse`
- `CategoryPromptsResponse`
- `CategoryWithOrganization`
- `CitationCategoriesResponse`
- `CitationCategory`
- `CitationCategoryFilter`
- `CitationDetail`
- `CitationDetailGroup`
- `CitationRow`
- `CitationScoreRef`
- `CitationsQuery`
- `CitationsResponse`
- `CitationsResult`
- `CitationsV2Info`
- `CitationsV2Query`
- `CitationsV2Response`
- `ClaimBreakdownQuery`
- `ClaimBreakdownResponse`
- `ClaimBreakdownRow`
- `ClaimCitationRow`
- `ClaimCitationSource`
- `ClaimCitationsQuery`
- `ClaimCitationsResponse`
- `ClaimEvidence`
- `ClaimModelOccurrence`
- `ClaimPromptBreakdownRow`
- `ClusterExampleRun`
- `ClusterExampleRunsQuery`
- `ClusterExampleRunsResponse`
- `ClusterModelShare`
- `ClusterVerificationPair`
- `ClusterVerificationPairsQuery`
- `ClusterVerificationPairsResponse`
- `Comment`
- `ContentAnalysis`
- `ContentFormat`
- `ContentOptimization`
- `ContentOptimizationAnalysisResponse`
- `ContentOptimizationResponse`
- `ContentOptimizationResponseInfo`
- `ContentType`
- `CreateAgentRequest`
- `CreateDocumentRequest`
- `CreateDocumentResponse`
- `CreateProjectRequest`
- `CreateProjectTaskRequest`
- `CreatePromptInput`
- `CreatePromptsBody`
- `CreatePromptsResponse`
- `CursorPagination`
- `DeleteDocumentRequest`
- `DeleteFolderRequest`
- `DimensionRef`
- `Document`
- `DocumentContent`
- `DocumentOperationResponse`
- `DomainWithOrganization`
- `EnabledFieldsResponse`
- `EntityFilterClause`
- `ExecutionCitationDetail`
- `FactCheckSetupStatusQuery`
- `FactCheckSetupStatusResponse`
- `FactcheckClaim`
- `FactcheckClaimsInfo`
- `FactcheckClaimsQuery`
- `FactcheckClaimsResponse`
- `FactcheckClaimsRow`
- `FactcheckScoreRow`
- `FactcheckScoresInfo`
- `FactcheckScoresQuery`
- `FactcheckScoresResponse`
- `FieldDiff`
- `FilterNode`
- `FolderOperationResponse`
- `HttpValidationError`
- `HostnameFilter`
- `IdOrName`
- `InaccuracyDriverRow`
- `InaccuracyDriversQuery`
- `InaccuracyDriversResponse`
- `InaccurateClusterRow`
- `InaccurateClustersQuery`
- `InaccurateClustersResponse`
- `InaccurateThemeRow`
- `InaccurateThemesQuery`
- `InaccurateThemesResponse`
- `ReportInfo`
- `InsightRow`
- `InsightsResponse`
- `Integration`
- `IntegrationsResponse`
- `KnowledgeBaseItem`
- `KnowledgeBaseSearchResponse`
- `KnowledgeBaseSearchResult`
- `ListAgentsResponse`
- `ListAgentsStatusFilter`
- `ListDocumentsResponse`
- `ListKnowledgeBasesResponse`
- `ListNodeTypesResponse`
- `ListProjectGenerationsResponse`
- `ListProjectTasksResponse`
- `ListProjectsResponse`
- `LiveGeneration`
- `MentionedFilter`
- `MerchantNameFilter`
- `ModelIdFilter`
- `NamedResource`
- `NamedResourceDiffList`
- `NodeSchemaResponse`
- `NodeTypeSummary`
- `NumericMetricFilter`
- `OptimizationQuery`
- `OrderByDirection`
- `Organization`
- `OrganizationAssetsResponse`
- `OrganizationPersonasWithCategory`
- `OrganizationPersonasWithCategoryResponse`
- `Pagination`
- `PaginationInfo`
- `PathFilter`
- `PersonaIdFilter`
- `PersonaProfile`
- `PersonaProfileBehavior`
- `PersonaProfileDemographics`
- `PersonaProfileEmployment`
- `ProductNameFilter`
- `Project`
- `ProjectAttachment`
- `ProjectDetail`
- `ProjectDetailResponse`
- `ProjectGeneration`
- `ProjectGenerationContext`
- `ProjectGenerationContextItem`
- `ProjectGenerationDateRange`
- `ProjectGenerationResponse`
- `ProjectGenerationStatus`
- `ProjectGenerationStatusResponse`
- `ProjectListItem`
- `ProjectResponse`
- `ProjectStatusPayload`
- `ProjectStatusResponse`
- `ProjectTask`
- `ProjectTaskDetail`
- `ProjectTaskDetailResponse`
- `ProjectTaskResponse`
- `ProjectTaskStatusPayload`
- `ProjectTaskStatusResponse`
- `Prompt`
- `PromptFilter`
- `PromptIdFilter`
- `PromptInput`
- `PromptOrderBy`
- `PromptPreview`
- `PromptType`
- `PromptTypeFilter`
- `PromptUpdatePreview`
- `QueryFanoutRow`
- `QueryFanoutsQuery`
- `QueryFanoutsV2Info`
- `QueryFanoutsV2Query`
- `QueryFanoutsV2Response`
- `ReadDocumentResponse`
- `Recommendation`
- `RecommendationImpact`
- `RecommendationStatus`
- `RecommendationSuggestion`
- `ReferralSourceFilter`
- `ReferralTypeFilter`
- `ReferralsQuery`
- `ReferralsQueryV2`
- `RegionIdFilter`
- `RegionNameFilter`
- `ReplaceDocumentContentRequest`
- `ReplaceDocumentContentResponse`
- `ReportResponse`
- `ReportResult`
- `RootDomainFilter`
- `RunAgentRequest`
- `SearchKnowledgeBaseFilters`
- `SearchKnowledgeBaseRequest`
- `SearchQueryFilter`
- `SentimentMetrics`
- `SentimentPeriodScores`
- `SentimentQuery`
- `SentimentRow`
- `SentimentScores`
- `SentimentTheme`
- `SentimentV2ClaimFilter`
- `SentimentV2ClaimIdFilter`
- `SentimentV2Info`
- `SentimentV2ModelIdFilter`
- `SentimentV2PersonaIdFilter`
- `SentimentV2PromptIdFilter`
- `SentimentV2Query`
- `SentimentV2RegionIdFilter`
- `SentimentV2ReportDataPoint`
- `SentimentV2ReportGroupMetadata`
- `SentimentV2ReportInfo`
- `SentimentV2ReportQuery`
- `SentimentV2ReportResponse`
- `SentimentV2Response`
- `SentimentV2RunIdFilter`
- `SentimentV2SentimentFilter`
- `SentimentV2TagIdFilter`
- `SentimentV2ThemeFilter`
- `SentimentV2ThemeIdFilter`
- `SentimentV2TopicIdFilter`
- `ShoppingBrandRow`
- `ShoppingBrandsV2Info`
- `ShoppingBrandsV2Query`
- `ShoppingBrandsV2Response`
- `ShoppingMerchantRow`
- `ShoppingMerchantsV2Info`
- `ShoppingMerchantsV2Query`
- `ShoppingMerchantsV2Response`
- `ShoppingProductRow`
- `ShoppingProductsV2Info`
- `ShoppingProductsV2Query`
- `ShoppingProductsV2Response`
- `ShoppingTriggerRateRow`
- `ShoppingTriggerRateV2Info`
- `ShoppingTriggerRateV2Query`
- `ShoppingTriggerRateV2Response`
- `StreamCitationsQuery`
- `StreamSentimentQuery`
- `StreamVisibilityQuery`
- `StreamWebSearchResultsQuery`
- `Tag`
- `TagIdFilter`
- `TagNameFilter`
- `ThemeFilter`
- `TopInaccurateClaimRow`
- `TopInaccurateClaimsQuery`
- `TopInaccurateClaimsResponse`
- `Topic`
- `TopicIdFilter`
- `TopicInput`
- `TopicNameFilter`
- `UpdateAgentRequest`
- `UpdateDocumentRequest`
- `UpdateProjectTaskRequest`
- `UpdateProjectTaskStatusRequest`
- `UpdatePromptInput`
- `UpdatePromptStatusBody`
- `UpdatePromptStatusResponse`
- `UpdatePromptsBody`
- `UpdatePromptsResponse`
- `UrlFilter`
- `UserInput`
- `ValidationError`
- `VisibilityQuery`
- `VisibilityRow`
- `VisibilityV2Info`
- `VisibilityV2Query`
- `VisibilityV2Response`
- `WebSearchResultsQuery`
- `WebSearchResultsResponse`
- `WebSearchResultsResult`
- `YoutubeChannelRow`
- `YoutubeChannelsInfo`
- `YoutubeChannelsQuery`
- `YoutubeChannelsResponse`
- `YoutubeCoverage`
- `YoutubeSummaryInfo`
- `YoutubeSummaryQuery`
- `YoutubeSummaryResponse`
- `YoutubeVideoRow`
- `YoutubeVideosInfo`
- `YoutubeVideosQuery`
- `YoutubeVideosResponse`
- `AppRoutesV2AnswerEngineInsightsReportsQueryFanoutsSortSpec`
- `AppRoutesV2AnswerEngineInsightsReportsSentimentSortSpec`
- `AppRoutesV2AnswerEngineInsightsReportsVisibilitySortSpec`
- `ProfoundAnswerEngineInsightsFiltersAssetNameFilter`
