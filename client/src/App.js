import './App.css';

import React, { useEffect, useState } from 'react';

function App() {
	const [wasm, setWasm] = useState();

	useEffect(() => {
		try {
			import("wasm-gl").then((loadedWasm) => {
                setWasm(loadedWasm);
                debugger 
                loadedWasm.do_it('Hey dawg')
			});
		} catch (err) {
			console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
		}
	}, []);

	return <div className="App"></div>;
}

export default App;
