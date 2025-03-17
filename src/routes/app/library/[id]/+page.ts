export async function load({ params }) {
	// if (Object.keys(get(roms)).includes(params.id)) {
  //   return {
  //     id: params.id
  //   }
  // }

  // error(404, 'Not found');

  return {
    id: params.id
  }
};