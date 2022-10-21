import { useGlobalContext } from 'context/global';
import React, { useState } from 'react';

function RankMenu() {
  const {
    state: { rank },
    setState,
  } = useGlobalContext();

  return (
    <select
      id="rank"
      defaultValue="platinum_plus"
      onChange={(e) => {
        setRank(e.target.value);
        exported.rank = e.target.value;
      }}
    >
      <option value="challenger">Challenger</option>
      <option value="grandmaster">Grandmaster</option>
      <option value="master">Master</option>
      <option value="diamond">Diamond</option>
      <option value="platinum">Platinum</option>
      <option value="gold">Gold</option>
      <option value="silver">Silver</option>
      <option value="bronze">Bronze</option>
      <option value="iron">Iron</option>
      <option value="overall">All Ranks</option>
      <option value="master_plus">Master +</option>
      <option value="diamond_plus">Diamond +</option>
      <option value="diamond_2_plus0">Diamond 2 +</option>
      <option value="platinum_plus">Platinum +</option>
    </select>
  );
}
