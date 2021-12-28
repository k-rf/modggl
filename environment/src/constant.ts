export const PRODUCT_NAME = 'modggly' as const;
export const PRODUCTION = `${PRODUCT_NAME}`;
export const STAGING = `${PRODUCT_NAME}-staging`;
export const PROJECT_NAME = `${PRODUCT_NAME}-pipeline`;
export const REGION = 'asia-northeast1' as const;
export const REGISTRY_NAME = `${PRODUCT_NAME}-artifact-registry`;

export type Environment = typeof PRODUCTION | typeof STAGING;
export type ProductName = typeof PRODUCT_NAME;
export type ProjectName = typeof PROJECT_NAME;
export type Region = typeof REGION;
export type RegistryName = typeof REGISTRY_NAME;
