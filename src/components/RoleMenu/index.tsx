import { SyntheticEvent } from 'react';

import { Paper, ToggleButtonGroup, ToggleButton, Box, Skeleton } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import type { AutoCompleteOption, Role } from 'interfaces';
import { Theme } from '@mui/material';
import { createArrayFromLength } from 'utils/utils';

interface RoleImageProps {
  role: Role;
  active: boolean;
}

function RoleImage(props: RoleImageProps) {
  const { role, active } = props;

  return (
    <img
      loading="lazy"
      width="30"
      src={`../common${role.localPath}`}
      srcSet={`../common${role.localPath} 2x`}
      alt={role.name}
      onError={({ currentTarget }) => {
        currentTarget.onerror = null;
        currentTarget.src = role.url;
        currentTarget.srcset = `${role.url} 2x`;
      }}
    />
  );
}

interface RoleMenuProps {
  loading: boolean;
}

function RoleMenu(props: RoleMenuProps) {
  const { loading } = props;

  const {
    state: { role: activeRole, roleList },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: string | null) => {
    const newValue = value || '';

    dispatch({ type: Actions.UPDATE_ROLE, payload: newValue });
  };

  return (
    <Paper elevation={3} sx={{ width: 'min-content' }}>
      <ToggleButtonGroup
        value={activeRole}
        disabled={loading}
        exclusive
        onChange={handleChangeRank}
        aria-label="role selection"
      >
        {loading
          ? createArrayFromLength(5).map((id) => (
              <ToggleButton key={`role-loading-${id}`} value={id} sx={{ padding: '5px' }}>
                <Skeleton variant="circular" animation="wave" width={30} height={30} />
              </ToggleButton>
            ))
          : roleList.map((role) => (
              <ToggleButton key={`role-${role.name}-${role.id}`} value={role.id} aria-label={role.name} sx={{ padding: '5px' }}>
                <RoleImage role={role} active={role.id === activeRole} />
              </ToggleButton>
            ))}
      </ToggleButtonGroup>
    </Paper>
  );
}

export default RoleMenu;
