// Simulator-only regression: use prepare_sleep_timer_fixture.py in a temporary signing/test project.
import XCTest
final class OriginalPortUITests: XCTestCase {
 func testSleepTimer24() { checkSleepTimer("24") }
 func testSleepTimer120() { checkSleepTimer("120") }
 func checkSleepTimer(_ fps:String) {
  let app = XCUIApplication()
  app.launchEnvironment = ["CHUZZLE_RENDER_SCALE":"1", "KOI_MUTE":"1", "KOI_GAME_FPS":fps]
  app.launch()
  sleep(15)
  let f=app.frame
  let origin=app.coordinate(withNormalizedOffset: .zero)
  func point(_ x:CGFloat,_ y:CGFloat)->XCUICoordinate {
   origin.withOffset(CGVector(dx:x*f.width/320,dy:(f.height-f.width*1.5)/2+y*f.width/320))
  }
  point(304,464).tap()
  sleep(3)
  point(117,457).press(forDuration:0.1,thenDragTo:point(132,457))
  sleep(2)
  let set=XCTAttachment(screenshot:app.screenshot());set.name="Timer selected";set.lifetime = .keepAlways;add(set)
  point(160,60).tap()
  sleep(25)
  XCTAssertEqual(app.state,.runningForeground,"Timer must no longer destroy/exit the pond")
  let expired=XCTAttachment(screenshot:app.screenshot());expired.name="After expiration";expired.lifetime = .keepAlways;add(expired)
  XCUIDevice.shared.press(.home)
  sleep(2)
  app.activate()
  sleep(2)
  // With Auto-Lock allowed, returning need not keep the app foreground.
  // It must remain alive, and direct interaction below must resume it.
  XCTAssertNotEqual(app.state,.notRunning)
  point(160,240).tap()
  sleep(3)
  XCTAssertEqual(app.state,.runningForeground,"Touch must resume the retained pond")
 }
}
