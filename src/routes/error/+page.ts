export async function load({ params, url }) {
  let message = url.searchParams.get('message');
  let fix = url.searchParams.get('fix');
  
  return { message, fix };
}