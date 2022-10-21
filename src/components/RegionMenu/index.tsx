function RegionMenu() {
  const [region, setRegion] = useState<string>('world');
  const element = (
    <select
      id="region"
      defaultValue="world"
      onChange={(e) => {
        setRegion(e.target.value);
        exported.region = e.target.value;
      }}
    >
      <option value="world">World</option>
      <option value="na1">North America</option>
      <option value="euw1">EU West</option>
      <option value="kr">Korea</option>
      <option value="br1">Brazil</option>
      <option value="eun1">EU North</option>
      <option value="jp1">Japan</option>
      <option value="la1">LA North</option>
      <option value="la2">LA South</option>
      <option value="oc1">OCE</option>
      <option value="ru">Russia</option>
      <option value="tr1">Turkey</option>
    </select>
  );
  console.log(region);
  return element;
}
