import {
  REGION,
  PIPELINE_PROJECT,
  PRODUCT_NAME,
  PRODUCTION_PROJECT,
} from '../constant';
import { EnvService } from '../lib/env.service';

import { createRegistry } from './module/registry';
import { Run } from './module/run';
import { Trigger } from './module/trigger';

const envService = new EnvService();

const { registry, registryName } = createRegistry({
  productName: PRODUCT_NAME,
  region: REGION,
});
export const registryId = registry.id;

const triggerBuilder = new Trigger(envService, {
  region: REGION,
  pipelineProject: PIPELINE_PROJECT,
  productName: PRODUCT_NAME,
  registryName,
});

const productionRunBuilder = new Run(envService, {
  environment: PRODUCTION_PROJECT,
  image: triggerBuilder.image,
});
const productionRun = productionRunBuilder.create();
export const productionRunId = productionRun.id;

const trigger = triggerBuilder.create();
export const triggerId = trigger.id;
