import * as gcp from '@pulumi/gcp';

import { ProductName, Region } from '../../constant';

type Props = {
  productName: ProductName;
  region: Region;
};

export const createRegistry = ({ productName, region }: Props) => {
  const registryName = `${productName}-artifact-registry`;

  const registry = new gcp.artifactregistry.Repository(registryName, {
    format: 'Docker',
    location: region,
    repositoryId: registryName,
    description: 'The artifact registry for modggly',
    project: 'modggly-pipeline',
  });

  return { registry, registryName };
};
