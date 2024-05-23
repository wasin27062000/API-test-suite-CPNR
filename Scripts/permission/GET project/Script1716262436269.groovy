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

response = WS.sendRequest(findTestObject('Permision/GET project', [('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'status.cause', null)

id_permission = WS.getElementPropertyValue(response, 'response.data[0].id')

id_project = WS.getElementPropertyValue(response, 'response.data[0].project.id')

iconid = WS.getElementPropertyValue(response, 'response.data[0].project.iconId')

projectname_th = WS.getElementPropertyValue(response, 'response.data[0].project.name.th')

projectname_en = WS.getElementPropertyValue(response, 'response.data[0].project.name.en')

GlobalVariable.id_permission = id_permission

println('id_permission : ' + id_permission)

GlobalVariable.id_project = id_project

println('id_project : ' + id_project)

GlobalVariable.iconid = iconid

println('iconid : ' + iconid)

GlobalVariable.projectname_th = projectname_th

println('projectname_th : ' + projectname_th)

GlobalVariable.projectname_en = projectname_en

println('projectname_en : ' + projectname_en)

