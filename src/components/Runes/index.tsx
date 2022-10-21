function Runes() {
  const [page, setPage] = useState<Array<Array<string | null>>>([
    [null, null, null, null],
    [null, null],
  ]);

  return (
    <div id="get-runes">
      <button
        onClick={() => {
          runes(setPage);
        }}
      >
        Click Me
      </button>
      <div>
        {page[0][0]} <br />
        {page[0][1]} <br />
        {page[0][2]} <br />
        {page[0][3]}
      </div>
    </div>
  );
}
