import * as path from 'path';

import * as dotenv from 'dotenv';

type EnvironmentVariable = {
  name: string;
  value: string;
};

export class EnvService {
  constructor() {
    const envPath = path.join(__dirname, '../../.env.local');
    console.log(envPath);
    dotenv.config({ path: envPath });
  }

  private getEnv(name: string): EnvironmentVariable {
    const value = process.env[name];

    if (value) {
      return { name, value };
    } else {
      throw new Error(`${name} is not defined.`);
    }
  }

  get githubOwner(): EnvironmentVariable {
    return this.getEnv('GITHUB_OWNER');
  }

  get githubRepository(): EnvironmentVariable {
    return this.getEnv('GITHUB_REPOSITORY');
  }

  get githubBranch(): EnvironmentVariable {
    return this.getEnv('GITHUB_BRANCH');
  }

  get serviceAccountProduction(): EnvironmentVariable {
    return this.getEnv('SERVICE_ACCOUNT_PRODUCTION');
  }

  get serviceAccountRunProduction(): EnvironmentVariable {
    return this.getEnv('SERVICE_ACCOUNT_RUN_PRODUCTION');
  }

  get serviceAccountStaging(): EnvironmentVariable {
    return this.getEnv('SERVICE_ACCOUNT_STAGING');
  }

  get serviceAccountRunStaging(): EnvironmentVariable {
    return this.getEnv('SERVICE_ACCOUNT_RUN_STAGING');
  }

  get togglApi(): EnvironmentVariable {
    return this.getEnv('TOGGL_API');
  }

  get reportApi(): EnvironmentVariable {
    return this.getEnv('REPORT_API');
  }

  get workspaceId(): EnvironmentVariable {
    return this.getEnv('WORKSPACE_ID');
  }

  get userAgent(): EnvironmentVariable {
    return this.getEnv('USER_AGENT');
  }

  get token(): EnvironmentVariable {
    return this.getEnv('TOKEN');
  }

  get tokenType(): EnvironmentVariable {
    return this.getEnv('TOKEN_TYPE');
  }

  get slackWebHookUrl(): EnvironmentVariable {
    return this.getEnv('SLACK_WEB_HOOK_URL');
  }
}
