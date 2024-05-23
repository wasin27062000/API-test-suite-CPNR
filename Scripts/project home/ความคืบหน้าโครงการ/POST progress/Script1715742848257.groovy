import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('project home/ความคืบหน้าโครงการ/POST progress', [('token') : GlobalVariable.token
            , ('id_project') : GlobalVariable.id_project]))

WS.verifyResponseStatusCode(response, 200)

id_progress = WS.getElementPropertyValue(response, 'response.id')

nameth_progress = WS.getElementPropertyValue(response, 'response.name.th')

nameen_progress = WS.getElementPropertyValue(response, 'response.name.en')

id_itemprogress = WS.getElementPropertyValue(response, 'response.items[0].id')

nameth_itemprogress = WS.getElementPropertyValue(response, 'response.items[0].name.th')

nameen_itemprogress = WS.getElementPropertyValue(response, 'response.items[0].name.en')

descth_itemprogress = WS.getElementPropertyValue(response, 'response.items[0].description.th')

descen_itemprogress = WS.getElementPropertyValue(response, 'response.items[0].description.en')

GlobalVariable.id_progress = id_progress

println('id_progress : ' + id_progress)

GlobalVariable.nameth_progress = nameth_progress

println('nameth_progress : ' + nameth_progress)

GlobalVariable.nameen_progress = nameen_progress

println('nameen_progress : ' + nameen_progress)

GlobalVariable.id_itemprogress = id_itemprogress

println('id_itemprogress : ' + id_itemprogress)

GlobalVariable.nameth_itemprogress = nameth_itemprogress

println('nameth_itemprogress : ' + nameth_itemprogress)

GlobalVariable.nameen_itemprogress = nameen_itemprogress

println('nameen_itemprogress : ' + nameen_itemprogress)

GlobalVariable.descth_itemprogress = descth_itemprogress

println('descth_itemprogress : ' + descth_itemprogress)

GlobalVariable.descen_itemprogress = descen_itemprogress

println('descen_itemprogress : ' + descen_itemprogress)
