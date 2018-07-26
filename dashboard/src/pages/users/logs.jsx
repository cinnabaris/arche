import Link from 'umi/link';
import Head from "../../components/Head"

export default () => (<div>
  <Head title={{id: "users sign in"}}/>
  <Link to="/users/sign-up">sign up</Link>
  sign in
</div>)