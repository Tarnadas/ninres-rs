import React, { FC } from 'react';

import { Grid } from '@geist-ui/react';

export const Badge: FC<{
  data: string;
}> = ({ data }) => {
  return (
    <Grid>
      <object data={data} type="image/svg+xml" />
    </Grid>
  );
};
