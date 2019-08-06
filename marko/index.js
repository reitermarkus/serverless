import UIkit from 'uikit'
import Icons from 'uikit/dist/js/uikit-icons'
import UI from './index.marko'

UIkit.use(Icons)

UI.renderSync().appendTo(document.body)
