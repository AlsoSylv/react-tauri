import { ReactNode, useState } from 'react';

import { Unstable_Grid2 as Grid, Avatar, Typography } from '@mui/material';
import { usePopupState, bindHover, bindPopover } from 'material-ui-popup-state/hooks';
import HoverPopover from 'material-ui-popup-state/HoverPopover';

import type { RuneData, Shard } from 'interfaces';

function Rune({ name, localImage, image, active }: RuneData | Shard): ReactNode {
  const popupState = usePopupState({ variant: 'popover', popupId: `rune-${name}` });
  const [url, setUrl] = useState(localImage ? `../runes${localImage}` : image);

  return (
    <Grid key={name} sm sx={{ display: 'flex', alignSelf: 'center', justifyContent: 'center' }}>
      <Avatar
        src={url}
        alt={name}
        sx={{ ...(!active && { filter: 'grayscale(100%)', opacity: '.3' }) }}
        {...bindHover(popupState)}
        imgProps={{
          onError: () => setUrl(image),
        }}
      />
      <HoverPopover
        {...bindPopover(popupState)}
        anchorOrigin={{
          vertical: 'top',
          horizontal: 'center',
        }}
        transformOrigin={{
          vertical: 'bottom',
          horizontal: 'center',
        }}
      >
        <Typography sx={{ margin: '10px' }}>{name}</Typography>
      </HoverPopover>
    </Grid>
  );
}

export default Rune;
