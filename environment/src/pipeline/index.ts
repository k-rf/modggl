import { REGION, PROJECT_NAME, PRODUCT_NAME } from '../constant';

import { createRegistry } from './module/registry';
import { createTrigger } from './module/trigger';

const { registry, registryName } = createRegistry({
  productName: PRODUCT_NAME,
  region: REGION,
});
export const registryId = registry.id;

const trigger = createTrigger({
  region: REGION,
  projectName: PROJECT_NAME,
  productName: PRODUCT_NAME,
  registryName,
});
export const triggerId = trigger.id;
