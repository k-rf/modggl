export const PRODUCT_NAME = 'modggl' as const;
export const PRODUCTION_PROJECT = `${PRODUCT_NAME}-production`;
export const STAGING_PROJECT = `${PRODUCT_NAME}-staging`;
export const PIPELINE_PROJECT = `${PRODUCT_NAME}-pipeline`;
export const REGION = 'asia-northeast1' as const;
export const REGISTRY_NAME = `${PRODUCT_NAME}-artifact-registry`;
export const RUN_NAME = `${PRODUCT_NAME}-run`;

export type Environment = typeof PRODUCTION_PROJECT | typeof STAGING_PROJECT;
export type ProductName = typeof PRODUCT_NAME;
export type PipelineProject = typeof PIPELINE_PROJECT;
export type Region = typeof REGION;
export type RegistryName = typeof REGISTRY_NAME;
export type RunName = typeof RUN_NAME;
