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

response = WS.sendRequest(findTestObject('project home/UPDATE project Not input phone juristic', [('token') : GlobalVariable.token
            , ('id_project') : GlobalVariable.id_project, ('id_progress') : GlobalVariable.id_progress, ('nameth_progress') : GlobalVariable.nameth_progress
            , ('nameen_progress') : GlobalVariable.nameen_progress, ('id_itemprogress') : GlobalVariable.id_itemprogress
            , ('nameth_itemprogress') : GlobalVariable.nameth_itemprogress, ('nameen_itemprogress') : GlobalVariable.nameen_itemprogress
            , ('descth_itemprogress') : GlobalVariable.descth_itemprogress, ('descen_itemprogress') : GlobalVariable.descen_itemprogress
            , ('id_plan') : GlobalVariable.id_plan, ('id_planitem') : GlobalVariable.id_planitem, ('id_categoryproject') : GlobalVariable.id_categoryproject
            , ('nameth_categoryproject') : GlobalVariable.nameth_categoryproject, ('nameen_categoryproject') : GlobalVariable.nameen_categoryproject
            , ('id_skytrain') : GlobalVariable.id_skytrain, ('nameth_skytrain') : GlobalVariable.nameth_skytrain, ('nameen_skytrain') : GlobalVariable.nameen_skytrain]))

WS.verifyElementPropertyValue(response, 'status.cause', '00')

