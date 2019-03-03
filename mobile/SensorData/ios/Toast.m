#import "React/RCTBridgeModule.h"

@interface RCT_EXTERN_REMAP_MODULE(ToastIOS, Toast, NSObject)

RCT_EXTERN_METHOD(show: (nonnull NSString)message duration:(nonnull NSNumber)duration)

@end
