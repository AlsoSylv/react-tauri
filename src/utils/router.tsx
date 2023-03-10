import { createBrowserRouter, createRoutesFromElements, Route } from 'react-router-dom';

import Wrapper from 'components/Wrapper';
import Champion from 'pages/Champion';
import MainPage from 'pages/MainPage';

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<Wrapper />}>
      <Route index element={<MainPage />} />
      <Route path="/champions/:champion" element={<Champion />} />
    </Route>
  )
);

export default router;
