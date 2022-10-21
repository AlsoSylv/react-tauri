function RoleMenu() {
  const [role, setRole] = useState<string>('');

  console.log(role);

  return (
    <select
      id="roles"
      defaultValue="none"
      onChange={(e) => {
        setRole(e.target.value);
        exported.role = e.target.value;
      }}
    >
      <option value="none" disabled>
        None
      </option>
      <option value="top">Top</option>
      <option value="jungle">Jungle</option>
      <option value="mid">Mid</option>
      <option value="adc">ADC</option>
      <option value="support">Support</option>
    </select>
  );
}
