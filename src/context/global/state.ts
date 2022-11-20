import { State } from 'interfaces';
import { getInitialData } from 'utils/utils';

const initialData = await getInitialData();

const initialGlobalState: State = {
  rank: 'Platinum Plus',
  role: 'Default',
  region: 'World',
  champion: null,
  championList: [],
  ...initialData,
};

export default initialGlobalState;
