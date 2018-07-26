import {
  Helmet
} from "react-helmet"
import PropTypes from 'prop-types'

const Widget = ({
  title
}) => (<Helmet>
  <title>{title.id}|subhead|title</title>
</Helmet>)


Widget.propTypes = {
  title: PropTypes.object.isRequired,
};

export default Widget