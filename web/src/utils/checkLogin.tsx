export default function checkLogin() {
  return localStorage.getItem('Authorization') !== null;
}
