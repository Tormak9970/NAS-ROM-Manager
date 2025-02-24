export async function load({ params, url }) {
  let message = url.searchParams.get('message');
  
  return { message };
}