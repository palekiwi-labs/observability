# Session

The agent uses Session with the following type:

```ts
export type Session = {
  id: string;
  projectID: string;
  directory: string;
  parentID?: string;
  share?: {
    url: string;
  };
  title: string;
  version: string;
  time: {
    created: number;
    updated: number;
  };
  revert?: {
    messageID: string;
    partID?: string;
    snapshot?: string;
    diff?: string;
  };
};
```
